use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub title: String,
    pub html_url: String,
    pub created_at: String,
    pub pull_request: Option<PullRequest>,
}

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    html_url: String,
}
