use serde::{Deserialize, Serialize};

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
                println!("{}", repository.name);
                println!("{}", repository.contributors_url);
                match github_api_client
                    .get::<Vec<Contributor>, &reqwest::Url, ()>(&repository.contributors_url, None::<&()>)
                    .await
                {
                    Ok(contributors) => {
                        for contributor in contributors {
                            println!("contributor {:?} contributed {:?}", contributor.login, contributor.contributions);
                        }
                        println!();
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


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    login: String,
    contributions: u16,
}