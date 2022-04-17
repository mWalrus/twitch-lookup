use anyhow::Result;
use reqwest::get;

pub async fn follow_age(user: &str, channel: &str) -> Result<String> {
    let followage = get(format!(
        "https://decapi.me/twitch/followage/{channel}/{user}"
    ))
    .await?
    .text()
    .await?;
    Ok(followage)
}

pub async fn title(channel: String) -> Result<String> {
    let title = get(format!("https://decapi.me/twitch/status/{channel}"))
        .await?
        .text()
        .await?;
    Ok(title)
}
