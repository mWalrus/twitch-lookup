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

pub async fn title(channel: &str) -> Result<String> {
    let title = get(format!("https://decapi.me/twitch/status/{channel}"))
        .await?
        .text()
        .await?;
    Ok(title)
}

pub async fn is_live(channel: &str) -> Option<String> {
    let response = get(format!("https://decapi.me/twitch/viewercount/{channel}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    if response.contains("offline") {
        return None;
    }
    Some(response)
}

pub async fn last_stream(channel: &str) -> Option<String> {
    let res = get(format!(
        "https://decapi.me/twitch/videos/{channel}?video_format=${{url}}"
    ))
    .await
    .unwrap()
    .text()
    .await
    .unwrap();
    if res.eq("Reached the end of the video list!") {
        return None;
    }
    Some(res)
}
