pub mod user;

use crate::Config;
use anyhow::Result;
use reqwest::{header, Client};
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

    pub async fn get_user(&self, user: &str) -> Result<User> {
        // FIXME: something is going wrong with the auth token.
        //        API response => 401 Invalid OAuth token
        let response = self
            .client
            .get(format!("https://api.twitch.tv/helix/users?login={user}"))
            .send()
            .await?
            .json::<UserData>()
            .await?;
        let user: User = response.data[0].clone();
        Ok(user)
    }
}
