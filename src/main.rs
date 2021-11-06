mod models;
mod error;
mod utils;

use models::App;
use crate::error::AppError;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    //try to parse cli arguments (<language> and <project_count>) into App struct.
    let args = App::from_args();
    //PAT for authentication
    let token = std::env::var("GITHUB_TOKEN")?;
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
                utils::fetch_results(&github_api_client, &project.contributors_url, &project.name).await;
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
