mod github;
mod telegram;
use log::info;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::{fs::File, sync::Arc};
use tokio::sync::Mutex;

use github::service::GithubService;
use telegram::service::TelegramService;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

const REPO: &str = "paradigmxyz/reth";

fn setup_logger() {
    std::fs::create_dir_all("logs").unwrap_or_else(|e| {
        eprintln!("Warning: Could not create logs directory: {}", e);
    });

    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("logs/github_bot.log").expect("Failed to create log file"),
    )])
    .expect("Failed to initialize logger");
}

#[tokio::main]
async fn main() -> Result<(), JobSchedulerError> {
    setup_logger();
    info!("Starting GitHub-Telegram bot application");

    let github_service = Arc::new(Mutex::new(GithubService::new(REPO).await));
    let telegram_client = Arc::new(Mutex::new(TelegramService::new().await));

    let mut scheduler = JobScheduler::new().await?;

    scheduler
        .add(Job::new_async("0 0/2 * * * *", move |_, _| {
            let github_service = github_service.clone();
            let telegram_client = telegram_client.clone();
            Box::pin(async move {
                let issues = github_service.lock().await.get_new_issues().await.unwrap();
                info!("Found {} issues", issues.len());
                if issues.is_empty() {
                    return;
                }
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
