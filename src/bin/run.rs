use anyhow::Context;
use github_notifier::{config, github};

///  Runs once, checking the GitHub API for new notifications.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let args = std::env::args().collect::<Vec<_>>();

    let file_path = if args.len() == 2 {
        args[1].clone()
    } else {
        std::env::var("HOME").unwrap() + "/.config/github-notifier/config.toml"
    };

    let config = config::Config::load_from_file(&file_path);

    let github_client = github::GithubClient::new(
        reqwest::Client::new(),
        &config.github_username,
        &config.github_token,
    );

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
