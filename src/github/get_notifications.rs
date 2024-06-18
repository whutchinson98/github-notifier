use crate::github::GITHUB_URL;

#[tracing::instrument(skip(client), level = "trace")]
pub(in crate::github) async fn get_notifications(
    client: &reqwest::Client,
    github_username: &str,
    github_token: &str,
) -> anyhow::Result<Vec<Notification>> {
    let res = client
        .get(format!("{GITHUB_URL}/notifications"))
        // TODO: setup last-modified header
        // If-Modified-Since: Thu, 25 Oct 2012 15:16:27 GMT
        .header("Accept", "application/json")
        .header("User-Agent", github_username)
        .header("Authorization", format!("Bearer {github_token}"))
        .send()
        .await?;

    let status = res.status();
    if !status.is_success() {
        match status {
            reqwest::StatusCode::NOT_MODIFIED => {
                tracing::trace!("not modified, updating header");
                return Ok(vec![]);
            }
            _ => {
                tracing::error!(status=?status, "failed to get notifications");
                return Err(anyhow::anyhow!(
                    "status:{} response:{}",
                    status,
                    res.text().await?
                ));
            }
        }
    }

    let text = res.text().await?;

    let result: Vec<Notification> = match serde_json::from_str(&text) {
        Ok(result) => result,
        Err(err) => {
            tracing::error!(error=?err, status=?status, "failed to parse response {:?}", text);
            return Err(anyhow::Error::msg("failed to parse response"));
        }
    };

    Ok(result)
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Notification {
    pub id: String,
    pub unread: bool,
    pub reason: String,
}
