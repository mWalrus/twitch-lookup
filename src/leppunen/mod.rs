pub mod user;

use anyhow::anyhow;
use anyhow::Result;
use reqwest::{get, StatusCode};
use user::User;

pub struct Api;

impl Api {
    pub async fn user(login: &str) -> Result<User> {
        if login.len() < 3 || login.len() > 25 {
            return Err(anyhow!("Invalid username"));
        }
        let url = format!("https://api.ivr.fi/v2/twitch/user/{login}");
        let user: User = get(url).await?.json().await?;
        Ok(user)
    }
    pub async fn is_valid_logs_query(user: &str, channel: &str) -> bool {
        let url = format!("https://logs.ivr.fi/list?channel={channel}&user={user}");
        let status = get(&url).await.unwrap().status();
        return status != StatusCode::NOT_FOUND;
    }
}
