use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //define github_api_client
    let github_api_client = octocrab::Octocrab::builder()
        //authenticate client with PAT
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;
    let language = std::env::var("LANGUAGE").unwrap();
    //assemble query to GitHub
    let query = &*("language:".to_owned() + language.as_str());
    let project_count = std::env::var("PROJECT_COUNT").unwrap().parse::<u32>().unwrap();
    //number of records per page fetched from GitHub per request
    let per_page = 50_u8;

    //fetch page of records
    let mut page = github_api_client
        .search()
        .repositories(query)
        .sort("stars")
        .order("desc")
        .per_page(per_page)
        .send()
        .await?;
    //number of processed repositories
    let mut processed_repos = 0u32;
    let mut switch: bool = true;

    while switch {
        for repository in &page {
            if processed_repos < project_count {
                match github_api_client
                    .get::<Vec<Contributor>, &reqwest::Url, ()>(&repository.contributors_url, None::<&()>)
                    .await
                {
                    Ok(contributors) => {
                        let mut contributors_map: HashMap<String, f64> = HashMap::new();
                        for contributor in contributors.iter() {
                            contributors_map.insert(contributor.login.clone(), contributor.contributions);
                            if contributors_map.len() >= 25 {
                                break;
                            }
                            // println!("contributor {:?} contributed {:?}", contributor.login, contributor.contributions);
                        }
                        let divider = contributors_map.values().sum::<f64>();
                        let result: Vec<_> = contributors_map
                            .iter()
                            .filter_map(|(x, y)| { if y / divider >= 0.75 { Some((x, y / divider)) } else { None } })
                            .collect();
                        if !result.is_empty() {
                            println!("project: {} user: {} percentage: {:.2}", repository.name, result.first().unwrap().0, result.first().unwrap().1)
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
    println!("{}", processed_repos);

    Ok(())
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    contributions: f64,
}