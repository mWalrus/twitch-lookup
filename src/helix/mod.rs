pub mod channel;
pub mod sub;
pub mod vod;

use crate::leppunen::Api;
use crate::Config;
use anyhow::Result;
use channel::Channel;
use reqwest::{header, Client};
use serde::Deserialize;
use sub::Sub;
use vod::Vod;

pub struct HelixClient {
    client: Client,
}

#[derive(Deserialize)]
pub struct HelixData<T> {
    data: Vec<T>,
    pagination: Pagination,
}

#[derive(Deserialize, Clone)]
pub struct Pagination {
    cursor: Option<String>,
}

impl Pagination {
    pub fn cursor(&self) -> Option<String> {
        self.cursor.clone()
    }
}

impl<T> HelixData<T>
where
    T: Clone,
{
    pub fn get_first(&self) -> &T {
        &self.data[0]
    }

    pub fn items(&self) -> Vec<T> {
        self.data.clone()
    }
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

    pub async fn subscription_status(&self, user: &str, channel: &str) -> Result<String> {
        // NOTE: if understood correctly, sub information about another user other than myself can only be acquired if that user has authenticated my application...
        //       gql might be the saviour here.
        // FIXME: doesnt seem to work with other users than myself
        //        check back here for info: https://dev.twitch.tv/docs/api/reference#get-broadcaster-subscriptions
        let user_id = Api::user(user).await?.uid();
        let channel_id = Api::user(channel).await?.uid();
        let res = self
            .client
            .get(format!(
                "https://api.twitch.tv/helix/subscriptions/user?broadcaster_id={}&user_id={}",
                &channel_id, &user_id
            ))
            .send()
            .await?;

        if let Ok(data) = res.json::<HelixData<Sub>>().await {
            let sub = data.get_first();
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
    pub async fn get_vods(&self, channel: &str, amount: Option<u8>) -> Option<Vec<Vod>> {
        let user_id = Api::user(channel).await.unwrap().uid();
        let res = self
            .client
            .get(format!(
                "https://api.twitch.tv/helix/videos?user_id={user_id}&first={}",
                amount.unwrap_or(1)
            ))
            .send()
            .await
            .unwrap()
            .json::<HelixData<Vod>>()
            .await
            .unwrap();
        Some(res.items())
    }

    pub async fn get_live_followed_channels(&self, user_id: u32) -> Option<Vec<Channel>> {
        let url = format!("https://api.twitch.tv/helix/streams/followed?user_id={user_id}");
        let res = self
            .client
            .get(url.clone())
            .send()
            .await
            .unwrap()
            .json::<HelixData<Channel>>()
            .await
            .unwrap();
        // TODO: pagination
        let mut items = res.items();
        if let Some(c) = res.pagination.cursor() {
            let mut new_url = url + &format!("after={c}");
            loop {
                let res = self
                    .client
                    .get(new_url.clone())
                    .send()
                    .await
                    .unwrap()
                    .json::<HelixData<Channel>>()
                    .await
                    .unwrap();
                items.append(&mut res.items());
                if let Some(new_c) = res.pagination.cursor() {
                    new_url = format!(
                        "https://api.twitch.tv/helix/streams/followed?user_id={user_id}&after={new_c}",
                    );
                } else {
                    break;
                }
            }
        }
        Some(res.items())
    }
}
