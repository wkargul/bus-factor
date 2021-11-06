use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{source}")]
    Other {
        #[from]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("{source}")]
    Client {
        #[from]
        source: octocrab::Error,
    },
    #[error("GITHUB_TOKEN env variable is required: {source}")]
    EnvVar {
        #[from]
        source: std::env::VarError,
    },
}
