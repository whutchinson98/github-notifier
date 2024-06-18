use anyhow::Context;
use github_notifier::github;

///  Runs once, checking the GitHub API for new notifications.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let github_token = std::env::var("GITHUB_TOKEN")
        .context("GITHUB_TOKEN not set")
        .unwrap();
    let github_username = std::env::var("GITHUB_USERNAME")
        .context("GITHUB_USERNAME not set")
        .unwrap();
    let github_client =
        github::GithubClient::new(reqwest::Client::new(), &github_username, &github_token);
    let notifications = match github_client.get_notifications().await {
        Ok(notifications) => notifications,
        Err(err) => {
            tracing::error!(error=?err, "failed to get notifications");
            std::process::exit(1);
        }
    };

    if notifications.is_empty() {
        std::process::exit(0);
    }

    std::process::exit(2);
}
