mod models;

use std::collections::HashMap;
use octocrab::Octocrab;
use reqwest::Url;
use models::{App, Contributor, KeyContributor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //try to parse cli arguments (<language> and <project_count>) into App struct.
    let args = App::from_args();
    //PAT for authentication
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    //define github_api_client
    let github_api_client = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()?;
    //assemble query to GitHub
    let query = format!("language:{}", args.language);
    //fetch page of records
    //by default setup in GitHubAPI it returns 30 records per page
    let mut current_page = github_api_client
        .search()
        .repositories(query.as_str())
        .sort("stars")
        .order("desc")
        .send()
        .await?;

    //take current set of repositories
    let mut projects = current_page.take_items();
    //number of processed repositories
    let mut processed_projects = 0u32;

    'read_records: while let Ok(Some(mut new_page)) = github_api_client.get_page(&current_page.next).await {
        projects.extend(new_page.take_items());

        for project in projects.drain(..) {
            if processed_projects < args.project_count {
                //get contributors as Vec<Contributor>
                fetch_contributors(&github_api_client, &project.contributors_url, &project.name).await;
                processed_projects += 1;
            } else {
                break 'read_records;
            }
        }
        current_page = new_page;
    }
    println!("\n{} projects have been analysed", processed_projects);
    Ok(())
}

async fn fetch_contributors(client: &Octocrab, route: &Url, project_name: &str) {
    match client
        .get::<Vec<Contributor>, &reqwest::Url, ()>(route, None::<&()>)
        .await {
        Ok(contributors) => { process(contributors, project_name) }
        Err(error) => println!("{:#?}", error)
    }
}

fn process(contributors: Vec<Contributor>, project_name: &str) {
    match analyse_contributors(contributors) {
        None => {}
        Some(key_contributor) => {
            print_stdout(project_name, key_contributor.user, key_contributor.percentage);
        }
    }
}

fn analyse_contributors(contributors: Vec<Contributor>) -> Option<KeyContributor> {
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
    if percentage >= 0.75 {
        Some(KeyContributor::new(login.to_string(), percentage))
    } else {
        None
    }
}

fn print_stdout(project_name: &str, user: String, percentage: f64) {
    println!("project: {:<20} user: {:<20} percentage: {:<.2}", project_name, user, percentage)
}
