#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Failed to request api: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Failed to extract embed link from url")]
    InvalidUrl(#[source] anyhow::Error),
}
