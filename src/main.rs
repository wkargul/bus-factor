use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //try to parse cli arguments into App struct.
    let args = App::from_args();
    //define github_api_client
    let github_api_client = octocrab::Octocrab::builder()
        //authenticate client with PAT
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;
    //assemble query to GitHub
    let query = &*("language:".to_owned() + args.language.as_str());
    //fetch page of records
    let mut page = github_api_client
        .search()
        .repositories(query)
        .sort("stars")
        .order("desc")
        .send()
        .await?;
    //number of processed repositories
    let mut processed_repos = 0u32;

    let mut switch: bool = true;
    while switch {
        for repository in &page {
            if processed_repos < args.project_count {
                match github_api_client
                    .get::<Vec<Contributor>, &reqwest::Url, ()>(&repository.contributors_url, None::<&()>)
                    .await
                {
                    Ok(contributors) => {
                        let contr_map: HashMap<String, f64> = contributors
                            .iter()
                            .take(25)
                            .map(|x| (x.login.clone(), x.contributions))
                            .collect();

                        let divider = contr_map.values().sum::<f64>();

                        let result: Vec<_> = contr_map
                            .iter()
                            .filter_map(|(x, y)| { if y / divider >= 0.75 { Some((x, y / divider)) } else { None } })
                            .collect();

                        if !result.is_empty() {
                            println!(
                                "project: {:<20} user: {:<20} percentage: {:<.2}",
                                repository.name,
                                result.first().unwrap().0,
                                result.first().unwrap().1
                            )
                        }
                    }
                    Err(error) => println!("{:#?}", error)
                }
                processed_repos += 1;
            } else {
                switch = false;
                break;
            }
        }

        if switch {
            page = match github_api_client
                .get_page::<octocrab::models::Repository>(&page.next)
                .await?
            {
                Some(next_page) => next_page,
                None => break,
            }
        }
    }
    println!("\n{} projects have been analyzed", processed_repos);

    Ok(())
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    contributions: f64,
}

#[derive(StructOpt, Debug)]
struct App {
    #[structopt(short, long)]
    language: String,
    #[structopt(short, long)]
    project_count: u32,
}
