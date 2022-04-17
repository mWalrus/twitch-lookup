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

pub async fn is_live(channel: String) -> Result<String> {
    let response = get(format!("https://decapi.me/twitch/viewercount/{channel}"))
        .await?
        .text()
        .await?;
    if response.contains("offline") {
        return Ok(response);
    } else {
        let formatted = format!("{channel} is currently live with {response} viewers");
        return Ok(formatted);
    }
}
