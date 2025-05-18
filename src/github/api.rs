use super::models::Issue;
use reqwest::Error;

pub struct GithubClient {
    client: reqwest::Client,
    repo: String,
}

impl GithubClient {
    pub async fn new(repo: &str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            repo: repo.to_string(),
        }
    }

    pub async fn get_recent_issues(&self, per_page: u32) -> Result<Vec<Issue>, Error> {
        let url = format!("https://api.github.com/repos/{}/issues", self.repo);
        let body = self
            .client
            .get(url)
            .header("User-Agent", "Awesome-github-bot")
            .query(&[
                ("direction", "desc"),
                ("per_page", per_page.to_string().as_str()),
                ("sort", "created"),
                ("state", "open"),
            ])
            .send()
            .await?
            .text()
            .await?;
        let issues: Vec<Issue> = serde_json::from_str(&body).unwrap_or_else(|e| {
            eprintln!("Error parsing issues: {}", e);
            vec![]
        });
        let issues_without_pr = issues
            .into_iter()
            .filter(|issue| issue.pull_request.is_none())
            .collect();
        Ok(issues_without_pr)
    }
}
