use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Issue {
    pub title: String,
    pub html_url: String,
    pub created_at: String,
    pub pull_request: Option<PullRequest>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PullRequest {
    pub html_url: String,
}
