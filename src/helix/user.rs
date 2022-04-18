use crate::format;
use colored::Colorize;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub data: Vec<User>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    broadcaster_type: String,
    login: String,
    display_name: String,
    #[serde(rename(deserialize = "id"))]
    uid: String,
    #[serde(rename(deserialize = "profile_image_url"))]
    profile_image: String,
    #[serde(rename(deserialize = "type"))]
    user_type: String,
    created_at: String,
    view_count: u32,
}

impl fmt::Display for User {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!(
            "{}{}",
            self.display_name.bold(),
            "'s profile information".bold()
        );
        println!("{} {}", "- UID:".bold(), self.uid.bold().magenta());
        println!(
            "{} {}",
            "- Created at:".bold(),
            self.created_at[..10].to_string().bold().green()
        );
        println!(
            "{} {}",
            "- Views:".bold(),
            format::readable_number(self.view_count).bold().magenta()
        );
        println!(
            "{} {}",
            "- Broadcaster type:".bold(),
            self.broadcaster_type.bold().blue()
        );
        println!("{} {}", "- User type:".bold(), self.user_type.bold().blue());
        println!(
            "{} {}",
            "- Profile image:".bold(),
            self.profile_image.bold().blue()
        );
        println!(
            "{} {}",
            "- Profile link:".bold(),
            format!("https://twitch.tv/{}", self.login).bold().blue()
        );
        Ok(())
    }
}

impl User {
    pub fn broadcaster_type(&self) -> String {
        self.broadcaster_type.to_string()
    }

    pub fn display_name(&self) -> String {
        self.display_name.to_string()
    }

    pub fn uid(&self) -> String {
        self.uid.to_string()
    }

    pub fn profile_image(&self) -> String {
        self.profile_image.to_string()
    }

    pub fn user_type(&self) -> String {
        self.user_type.to_string()
    }

    pub fn created_at(&self) -> String {
        // NOTE: maybe add real datetime handling
        self.created_at[..10].to_string()
    }

    pub fn login(&self) -> String {
        self.login.to_string()
    }

    pub fn view_count(&self) -> String {
        format::readable_number(self.view_count)
    }
}
