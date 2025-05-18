mod github;
mod telegram;
// use github::service::GithubService;
use telegram::service::TelegramService;

#[tokio::main]
async fn main() {
    // let github_service = GithubService::new("paradigmxyz/reth").await;

    // match github_service.get_new_issues().await {
    //     Ok(issues) => {
    //         println!("Found {} new issues", issues.len());
    //     }
    //     Err(e) => {
    //         eprintln!("Error fetching issues: {}", e);
    //     }
    // }
    let telegram_client = TelegramService::new().await;
    telegram_client
        .send_message("title\nhttps://github.com/paradigmxyz/reth/issues/16317")
        .await;
}
