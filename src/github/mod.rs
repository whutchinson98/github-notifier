use get_notifications::Notification;
#[allow(unused_imports)]
use mockall::automock;

mod get_notifications;

#[cfg(not(test))]
pub use GithubClientImpl as GithubClient;
#[cfg(test)]
pub use MockGithubClientImpl as GithubClient;

#[derive(Clone, Debug)]
pub struct GithubClientImpl {
    client: reqwest::Client,
    github_username: String,
    github_token: String,
}

pub const GITHUB_URL: &str = "https://api.github.com";

#[cfg_attr(test, automock)]
impl GithubClientImpl {
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
