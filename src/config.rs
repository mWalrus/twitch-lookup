use anyhow::Result;
use confy;
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use webbrowser;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    client_id: String,
    access_token: String,
}

impl Default for Config {
    fn default() -> Self {
        webbrowser::open("https://rusterino.waalrus.xyz/login").unwrap();
        let response: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Paste your credentials here")
            .interact()
            .unwrap();
        let mut split = response.split(';').into_iter();
        let access_token = split.next_back().unwrap().to_owned();
        let client_id = split.next_back().unwrap().to_owned();
        Self {
            client_id,
            access_token,
        }
    }
}

impl Config {
    pub fn read() -> Result<Self, confy::ConfyError> {
        // FIXME: fires twice if config needs to be constructed
        confy::load::<Config>("twitch-lookup")
    }

    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn _save(&self) -> Result<()> {
        confy::store("twitch-lookup", self).unwrap();
        Ok(())
    }
}
