use octocrab::Octocrab;
use reqwest::Url;
use std::collections::HashMap;
use crate::models::{Contributor, KeyContributor};

pub async fn fetch_results(client: &Octocrab, route: &Url, project_name: &str) {
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
