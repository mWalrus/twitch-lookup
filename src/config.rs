use anyhow::Result;
use confy;
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};
use webbrowser;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Config {
    client_id: String,
    access_token: String,
}

impl Config {
    pub fn read() -> Result<Self> {
        // FIXME: fires twice if config needs to be constructed
        let cfg = confy::load::<Config>("twitch-lookup")?;
        if cfg == Config::default() {
            webbrowser::open("https://rusterino.waalrus.xyz/login").unwrap();
            let response: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Paste your credentials here")
                .interact()
                .unwrap();
            let mut split = response.split(';').into_iter();
            let access_token = split.next_back().unwrap().to_owned();
            let client_id = split.next_back().unwrap().to_owned();
            let cfg = Self {
                client_id,
                access_token,
            };
            confy::store("twitch-lookup", &cfg).unwrap();
            Ok(cfg)
        } else {
            Ok(cfg)
        }
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
