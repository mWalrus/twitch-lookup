use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Config {
    pub login: String,
    pub user_id: String,
    pub client_id: String,
    pub access_token: String,
}

impl Config {
    pub fn read() -> Result<Self> {
        let cfg = confy::load::<Config>("twitch-lookup")?;
        if cfg == Config::default() {
            webbrowser::open("https://rusterino.waalrus.xyz/login").unwrap();
            let response = prompt("Paste your credentials here");
            let mut split = response.split(';');
            let login = split.next().unwrap().to_owned();
            let user_id = split.next().unwrap().to_owned();
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

    pub fn _save(&self) -> Result<()> {
        confy::store("twitch-lookup", self).unwrap();
        Ok(())
    }
}

fn prompt(msg: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact()
        .unwrap()
}
