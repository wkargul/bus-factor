use octocrab::Octocrab;
use reqwest::Url;
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
    //collect contributors in Vec -> <user: String, contributions: f64>
    let contr: Vec<(String, f64)> = contributors
        .iter()
        .take(25)
        .map(|contr| (contr.user.clone(), contr.contributions))
        .collect();

    //sum total number of contributions for further operation
    let total_project_contributions = contr.iter().fold(0_f64, |sum, c| sum + c.1);

    //create an option of KeyContributor meeting given conditions
    contr
        .iter()
        .filter_map(|(user, contributions)| bus_factor_check(user, contributions, total_project_contributions))
        .last()
}

fn bus_factor_check(user: &str, contributions: &f64, total_project_contributions: f64) -> Option<KeyContributor> {
    let percentage = contributions / total_project_contributions;
    if percentage >= 0.75 {
        Some(KeyContributor::new(user.to_string(), percentage))
    } else {
        None
    }
}

fn print_stdout(project_name: &str, user: String, percentage: f64) {
    println!("project: {:<20} user: {:<20} percentage: {:<.2}", project_name, user, percentage)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_user() -> &'static str {
        "test_user"
    }

    fn get_project_name() -> &'static str {
        "test_project"
    }

    fn spawn_contributor(user: String, contributions: f64) -> Contributor {
        Contributor {
            user,
            contributions,
        }
    }

    fn get_contributors_bus_fac_not_eq_1() -> Vec<Contributor> {
        vec![
            spawn_contributor(get_user().to_string(), 1_f64),
            spawn_contributor(get_user().to_string(), 2_f64),
            spawn_contributor(get_user().to_string(), 3_f64),
            spawn_contributor(get_user().to_string(), 40_f64),
            spawn_contributor(get_user().to_string(), 54_f64),
        ]
    }


    #[test]
    fn test_print_stdout() {
        let percentage = 0.9432423432_f64;
        let expected = println!("project: test_project         user: test_user            percentage: 0.94");

        assert_eq!(print_stdout(get_project_name(), get_user().to_string(), percentage), expected);
    }

    #[test]
    fn test_bus_factor_check_none() {
        let contributions = &10_f64;
        let total_project_contributions = 100_f64;

        let expected = Option::None;

        assert_eq!(bus_factor_check(get_user(), contributions, total_project_contributions), expected)
    }

    #[test]
    fn test_bus_factor_check_some() {
        let contributions = &75_f64;
        let total_project_contributions = 100_f64;
        let percentage = contributions / total_project_contributions;

        let expected = Option::Some(KeyContributor::new(get_user().to_string(), percentage));

        assert_eq!(bus_factor_check(get_user(), contributions, total_project_contributions), expected)
    }

    #[test]
    fn test_analyse_contributors_some() {
        let main_contributor = spawn_contributor(get_user().to_string(), 94_f64);
        let contributors = vec![
            spawn_contributor(get_user().to_string(), 1_f64),
            spawn_contributor(get_user().to_string(), 2_f64),
            spawn_contributor(get_user().to_string(), 3_f64),
            main_contributor.clone(),
        ];
        let total_contributions = contributors.iter().fold(0_f64, |sum, c| sum + c.contributions);

        let expected = Some(KeyContributor::new(main_contributor.user, main_contributor.contributions / total_contributions));

        assert_eq!(analyse_contributors(contributors), expected);
    }

    #[test]
    fn test_analyse_contributors_none() {
        assert_eq!(analyse_contributors(get_contributors_bus_fac_not_eq_1()), None);
    }

    #[test]
    fn test_process_no_output_1() {
        let contributors = vec![];
        let expected = ();

        assert_eq!(process(contributors, get_project_name()), expected);
    }

    #[test]
    fn test_process_no_output_2() {
        let expected = ();

        assert_eq!(process(get_contributors_bus_fac_not_eq_1(), get_project_name()), expected);
    }

    #[test]
    fn test_process_some_output() {
        let contributors = vec![
            spawn_contributor(get_user().to_string(), 1_f64),
            spawn_contributor(get_user().to_string(), 2_f64),
            spawn_contributor(get_user().to_string(), 3_f64),
            spawn_contributor(get_user().to_string(), 94_f64),
        ];

        let expected = println!("project: test_project         user: test_user            percentage: 0.94");

        assert_eq!(process(contributors, get_project_name()), expected);
    }
}
