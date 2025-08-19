use get_notifications::Notification;
mod get_notifications;

#[derive(Clone, Debug)]
pub struct GithubClient {
    client: reqwest::Client,
    github_username: String,
    github_token: String,
}

pub const GITHUB_URL: &str = "https://api.github.com";

impl GithubClient {
    #[tracing::instrument(skip(client), level = "trace")]
    pub fn new(client: reqwest::Client, github_username: &str, github_token: &str) -> Self {
        Self {
            client,
            github_username: github_username.to_string(),
            github_token: github_token.to_string(),
        }
    }

    pub async fn get_notifications(&self) -> anyhow::Result<Vec<Notification>> {
        get_notifications::get_notifications(
            &self.client,
            self.github_username.as_str(),
            self.github_token.as_str(),
        )
        .await
    }
}
