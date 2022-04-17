pub mod sub;
pub mod user;

use crate::Config;
use anyhow::Result;
use reqwest::{header, Client};
use sub::{Sub, SubData};
use user::User;
use user::UserData;

pub struct HelixClient {
    client: Client,
}

impl HelixClient {
    pub fn new(config: Config) -> Self {
        let builder = Client::builder();
        let mut headers = header::HeaderMap::new();

        let bearer = format!("Bearer {}", config.access_token());
        let auth_value = header::HeaderValue::from_str(&bearer).unwrap();
        let client_id_value = header::HeaderValue::from_str(config.client_id()).unwrap();

        headers.insert(header::AUTHORIZATION, auth_value);
        headers.insert("Client-Id", client_id_value);

        let client = builder.default_headers(headers).build().unwrap();

        Self { client }
    }

    pub async fn get_user(&self, user: &str) -> Option<User> {
        let response = self
            .client
            .get(format!("https://api.twitch.tv/helix/users?login={user}"))
            .send()
            .await
            .unwrap()
            .json::<UserData>()
            .await
            .unwrap();
        // FIXME: errors if the user is not found
        if response.data.is_empty() {
            return None;
        }
        let user: User = response.data[0].clone();
        Some(user)
    }

    pub async fn subscription_status(&self, user: &str, channel: &str) -> Result<String> {
        // FIXME: doesnt seem to work with other users than myself
        //        check back here for info: https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions
        let user_id = self.get_user(user).await.unwrap().uid();
        let channel_id = self.get_user(channel).await.unwrap().uid();
        let response = self
            .client
            .get(format!(
                "https://api.twitch.tv/helix/subscriptions/user?broadcaster_id={}&user_id={}",
                &channel_id, &user_id
            ))
            .send()
            .await?;

        if let Ok(sub_data) = response.json::<SubData>().await {
            let sub = &sub_data.data[0];
            if sub.is_gift() {
                return Ok(format!(
                    "{user} is subscribed to {channel} with a Tier {} gifted sub from {}",
                    sub.tier(),
                    sub.gifter()
                ));
            } else {
                return Ok(format!(
                    "{user} is subscribed to {channel} with a Tier {} sub",
                    sub.tier()
                ));
            }
        } else {
            return Ok(format!("{user} is not subscribed to {channel}"));
        }
    }
}
