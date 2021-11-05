use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub contributions: f64,
}

#[derive(Debug)]
pub struct KeyContributor {
    pub user: String,
    pub percentage: f64,
}

impl KeyContributor {
    pub fn new(user: String, percentage: f64) -> KeyContributor {
        KeyContributor {
            user,
            percentage,
        }
    }
}

#[derive(StructOpt, Debug)]
pub struct App {
    #[structopt(short, long)]
    pub language: String,
    #[structopt(short, long)]
    pub project_count: u32,
}

impl App {
    pub fn from_args() ->App { <App as StructOpt>::from_args() }
}
