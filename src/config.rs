use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Config {
    login: String,
    user_id: u32,
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
            let mut split = response.split(';');
            let login = split.next().unwrap().to_owned();
            let user_id = split.next().unwrap().to_owned().parse::<u32>().unwrap();
            let client_id = split.next().unwrap().to_owned();
            let access_token = split.next().unwrap().to_owned();
            let cfg = Self {
                login,
                user_id,
                client_id,
                access_token,
            };
            confy::store("twitch-lookup", &cfg).unwrap();
            Ok(cfg)
        } else {
            Ok(cfg)
        }
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn user_id(&self) -> u32 {
        self.user_id
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
