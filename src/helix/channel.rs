use crate::deser::{deserialize_date_time, deserialize_stream_status};
use crate::format::duration_to_hms;
use chrono::{DateTime, Utc};
use colored::Colorize;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug, Clone)]
pub struct Channel {
    user_name: String,
    user_login: String,
    game_name: String,
    #[serde(
        rename(deserialize = "type"),
        deserialize_with = "deserialize_stream_status"
    )]
    live: bool,
    title: String,
    viewer_count: u32,
    #[serde(deserialize_with = "deserialize_date_time")]
    started_at: DateTime<Utc>,
}

impl fmt::Display for Channel {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uptime = Utc::now().signed_duration_since(self.started_at);
        if self.live {
            let out = format!(
                "► {} is live playing {} to {} viewers:\n{} {}\n{} {}\n{} {}{}",
                self.user_name,
                self.game_name,
                self.viewer_count.to_string().magenta(),
                "- Title:".white(),
                self.title.white(),
                "- Uptime:".white(),
                duration_to_hms(uptime).green(),
                "- URL:".white(),
                "https://twitch.tv/".blue(),
                self.user_login.blue()
            );
            println!("{}", out.bold().red());
        } else {
            println!(
                "{} {} {}",
                "■".bold(),
                self.user_name.bold(),
                "is offline.".bold()
            );
        }
        Ok(())
    }
}
