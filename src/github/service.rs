use reqwest::Error;

use super::api::GithubClient;
use super::models::Issue;

pub struct GithubService {
    client: GithubClient,
    last_issue: Option<Issue>,
}

impl GithubService {
    pub async fn new(repo: &str) -> Self {
        let client = GithubClient::new(repo).await;
        Self {
            client,
            last_issue: None,
        }
    }

    pub async fn get_new_issues(&self) -> Result<Vec<Issue>, Error> {
        let issues = self.client.get_recent_issues(10).await?;
        let index = match &self.last_issue {
            Some(last_issue) => issues
                .iter()
                .position(|issue| issue.html_url == last_issue.html_url),
            None => None,
        };
        let new_issues = match index {
            Some(index) => issues.into_iter().skip(index + 1).collect(),
            None => issues,
        };
        Ok(new_issues)
    }
}
