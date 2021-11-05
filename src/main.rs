mod models;

use std::collections::HashMap;
use models::{App, Contributor, KeyContributor};

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
        for project in &page {
            if processed_repos < args.project_count {
                match github_api_client
                    .get::<Vec<Contributor>, &reqwest::Url, ()>(&project.contributors_url, None::<&()>)
                    .await {
                    Ok(contributors) => {
                        match analyze_contributors(contributors) {
                            None => {}
                            Some(key_contributor) => {
                                print_stdout(&project.name, key_contributor.user, key_contributor.percentage);
                            }
                        };
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
                .await? {
                Some(next_page) => next_page,
                None => break,
            }
        }
    }
    println!("\n{} projects have been analyzed", processed_repos);

    Ok(())
}

fn analyze_contributors(contributors: Vec<Contributor>) -> Option<KeyContributor> {
    //collect contributors in HashMap -> <key: login, value: contributions>
    let contr_map: HashMap<String, f64> = contributors
        .iter()
        .take(25)
        .map(|contr| (contr.login.clone(), contr.contributions))
        .collect();

    //sum total number of contributions for further operation
    let total_project_contributions = contr_map.values().sum::<f64>();

    //create an option of KeyContributor meeting given conditions
    contr_map
        .iter()
        .filter_map(|(login, contributions)| bus_factor_check(login, contributions, total_project_contributions))
        .last()
}

fn bus_factor_check(login: &str, contributions: &f64, total_project_contributions: f64) -> Option<KeyContributor> {
    let percentage = contributions / total_project_contributions;
    match percentage >= 0.75 {
        true => Some(KeyContributor::new(login.to_string(), percentage)),
        false => None
    }
}

fn print_stdout(project_name: &str, user: String, percentage: f64) {
    println!("project: {:<20} user: {:<20} percentage: {:<.2}", project_name, user, percentage)
}
