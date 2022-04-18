use crate::format;
use colored::Colorize;
use serde::Deserialize;
use std::fmt;

// TODO: https://dev.twitch.tv/docs/api/reference#get-videos
#[derive(Deserialize, Debug)]
pub struct VodData {
    pub data: Vec<Vod>,
}

#[derive(Deserialize, Debug)]
pub struct Vod {
    title: String,
    url: String,
    view_count: u32,
    #[serde(rename(deserialize = "type"))]
    vod_type: String,
    duration: String,
}

impl fmt::Display for Vod {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("{} {}", "- Title:".bold(), self.title.bold().green());
        println!("{} {}", "- URL:".bold(), self.url.bold().blue());
        println!(
            "{} {}",
            "- View count:".bold(),
            format::readable_number(self.view_count).bold().magenta()
        );
        println!("{} {}", "- Type:".bold(), self.vod_type.bold().green());
        println!("{} {}", "- Duration:".bold(), self.duration.bold().green());
        Ok(())
    }
}
