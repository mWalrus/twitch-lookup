pub mod channel;
pub mod vod;

use crate::leppunen::Api;
use crate::Config;
use channel::Channel;
use reqwest::{header, Client};
use serde::Deserialize;
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
    pub fn items(&self) -> Vec<T> {
        self.data.clone()
    }
}

impl HelixClient {
    pub fn new(config: &Config) -> Self {
        let builder = Client::builder();
        let mut headers = header::HeaderMap::new();

        let bearer = format!("Bearer {}", config.access_token);
        let auth_value = header::HeaderValue::from_str(&bearer).unwrap();
        let client_id_value = header::HeaderValue::from_str(&config.client_id).unwrap();

        headers.insert(header::AUTHORIZATION, auth_value);
        headers.insert("Client-Id", client_id_value);

        let client = builder.default_headers(headers).build().unwrap();

        Self { client }
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

    pub async fn get_live_followed_channels(&self, user_id: &str) -> Option<Vec<Channel>> {
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
