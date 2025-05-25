mod github;
mod telegram;
use std::sync::Arc;
use tokio::sync::Mutex;

use github::service::GithubService;
use telegram::service::TelegramService;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    let github_service = Arc::new(Mutex::new(GithubService::new("paradigmxyz/reth").await));
    let telegram_client = Arc::new(Mutex::new(TelegramService::new().await));

    let mut scheduler = JobScheduler::new().await?;

    scheduler
        .add(Job::new_async("0 0/1 * * * *", move |_, _| {
            let github_service = github_service.clone();
            let telegram_client = telegram_client.clone();
            Box::pin(async move {
                let issues = github_service.lock().await.get_new_issues().await.unwrap();
                let message = issues
                    .into_iter()
                    .map(|issue| format!("{}\n{}", issue.title, issue.html_url))
                    .collect::<Vec<String>>()
                    .join("\n");
                let _ = telegram_client.lock().await.send_message(message).await;
            })
        })?)
        .await?;

    scheduler.start().await?;

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c event");

    scheduler.shutdown().await?;
    Ok(())
}
