use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let github_api_client = octocrab::Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()?;
    let language = std::env::var("LANGUAGE").unwrap();
    let query = &*("language:".to_owned() + language.as_str());
    let project_count = std::env::var("PROJECT_COUNT").unwrap().parse::<u8>().unwrap();

    match github_api_client
        .search()
        .repositories(query)
        .sort("stars")
        .order("desc")
        .per_page(project_count)
        .send()
        .await
    {
        Ok(page) => {
            for repository in page {
                println!("{}", repository.name);
                println!("{}", repository.contributors_url);
                match github_api_client
                    .get::<Vec<Contributor>, reqwest::Url, ()>(repository.contributors_url, None::<&()>)
                    .await
                {
                    Ok(contributors) => {
                        for contributor in contributors {
                            println!("contributor {:?} contributed {:?}", contributor.login, contributor.contributions)
                        }
                        println!();
                    }
                    Err(error) => println!("{:#?}", error)
                }
            }
        }
        Err(error) => println!("{:#?}", error),
    }

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    contributions: u16,
}