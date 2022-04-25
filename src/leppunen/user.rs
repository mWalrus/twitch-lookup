use crate::deser::*;
use crate::format;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use colored::Colorize;
use serde::de::Deserializer;
use serde::Deserialize;
use std::fmt::Display;

pub trait CompactUser {
    fn print(&self) -> Result<()>;
}

pub trait VerboseUser {
    fn print(&self) -> Result<()>;
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    // FIXME: UID should not be deserialized into u32 since it can have leading zeroes.
    #[serde(rename(deserialize = "id"), deserialize_with = "deserialize_uid")]
    pub uid: u32,
    pub banned: bool,
    pub display_name: String,
    pub bio: Option<String>,
    pub follows: u16,
    pub followers: u32,
    #[serde(rename(deserialize = "profileViewCount"))]
    pub channel_views: u32,
    pub chat_color: String,
    pub logo: String,
    pub verified_bot: bool,
    #[serde(deserialize_with = "deserialize_date_time")]
    pub created_at: DateTime<Utc>,
    pub emote_prefix: String,
    pub roles: Roles,
    pub badges: Vec<Badge>,
    pub chat_settings: ChatSettings,
    pub last_broadcast: LastBroadcast,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Roles {
    pub is_affiliate: bool,
    pub is_partner: bool,
    pub is_staff: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Badge {
    pub title: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatSettings {
    #[serde(deserialize_with = "deserialize_millis")]
    pub chat_delay_ms: Duration,
    #[serde(deserialize_with = "deserialize_minutes")]
    pub followers_only_duration_minutes: Duration,
    #[serde(deserialize_with = "deserialize_seconds")]
    pub slow_mode_duration_seconds: Duration,
    #[serde(rename(deserialize = "isEmoteOnlyModeEnabled"))]
    pub emote_only: bool,
    #[serde(rename(deserialize = "isSubscribersOnlyModeEnabled"))]
    pub sub_only: bool,
    #[serde(rename(deserialize = "isUniqueChatModeEnabled"))]
    pub unique_chat: bool,
    pub require_verified_account: bool,
    #[serde(rename(deserialize = "rules"))]
    pub chat_rules: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastBroadcast {
    #[serde(deserialize_with = "deserialize_date_time")]
    pub started_at: DateTime<Utc>,
    pub title: Option<String>,
}

impl User {
    pub fn uid(&self) -> u32 {
        self.uid
    }
    pub fn display_name_colored(&self) -> String {
        let (r, g, b) = format::hex_to_rgb(&self.chat_color);
        self.display_name.truecolor(r, g, b).to_string()
    }
    pub fn following(&self) -> String {
        format::readable_number(self.follows as u32)
            .magenta()
            .to_string()
    }
    pub fn followers(&self) -> String {
        format::readable_number(self.followers)
            .magenta()
            .to_string()
    }
    pub fn channel_views(&self) -> String {
        format::readable_number(self.channel_views)
            .magenta()
            .to_string()
    }
    pub fn chat_color(&self) -> String {
        let (r, g, b) = format::hex_to_rgb(&self.chat_color);
        self.chat_color.truecolor(r, g, b).to_string()
    }
}

impl CompactUser for User {
    fn print(&self) -> Result<()> {
        let follows = format::readable_number(self.follows.into());
        let followers = format::readable_number(self.followers);
        let channel_views = format::readable_number(self.channel_views);

        println!(
            "{}{}",
            self.display_name_colored().bold(),
            "'s profile information:".bold()
        );
        println!(
            "{}{}",
            "- User ID:".bold(),
            self.uid.to_string().bold().magenta()
        );
        println!("{} {}", "- Banned:".bold(), yes_no(self.banned));
        println!(
            "{} {}",
            "- Display name:".bold(),
            self.display_name_colored().bold()
        );
        println!("{} {}", "- Follows:".bold(), follows.bold().magenta());
        println!("{} {}", "- Following:".bold(), followers.bold().magenta());
        println!(
            "{} {}",
            "- Channel view:".bold(),
            channel_views.bold().magenta()
        );
        println!("{} {}", "- Chat color:".bold(), self.chat_color().bold());
        println!("{} {}", "- Profile image:".bold(), self.logo.bold().blue());
        println!(
            "{} {}",
            "- Account created:".bold(),
            self.created_at
                .date()
                .naive_utc()
                .to_string()
                .bold()
                .green()
        );
        println!(
            "{} {}",
            "- Downtime:".bold(),
            self.last_broadcast.time_since().bold().green()
        );
        Ok(())
    }
}

impl VerboseUser for User {
    fn print(&self) -> Result<()> {
        let follows = format::readable_number(self.follows.into());
        let followers = format::readable_number(self.followers);
        let channel_views = format::readable_number(self.channel_views);

        println!(
            "{}{}",
            self.display_name_colored().bold(),
            "'s profile information:".bold()
        );
        println!(
            "{}{}",
            "- User ID:".bold(),
            self.uid.to_string().bold().magenta()
        );
        println!("{} {}", "- Banned:".bold(), yes_no(self.banned));
        println!(
            "{} {}",
            "- Bio:".bold(),
            self.bio.clone().unwrap_or("".to_string()).bold()
        );
        println!("{} {}", "- Follows:".bold(), follows.bold().magenta());
        println!("{} {}", "- Followers:".bold(), followers.bold().magenta());
        println!(
            "{} {}",
            "- Channel views:".bold(),
            channel_views.bold().magenta()
        );
        println!("{} {}", "- Chat color:".bold(), self.chat_color().bold());
        println!(
            "{} {}",
            "- Profile picture:".bold(),
            self.logo.bold().blue()
        );
        println!("{} {}", "- Verified bot:".bold(), yes_no(self.verified_bot));
        println!(
            "{} {}",
            "- Created at:".bold(),
            self.created_at
                .date()
                .naive_utc()
                .to_string()
                .bold()
                .green()
        );
        println!("{} {}", "- Emote prefix:".bold(), self.emote_prefix.bold());
        println!("{}\n{}", "- Roles:".bold(), self.roles);
        println!("{}", "- Badges:".bold());
        for badge in self.badges.iter() {
            println!("{badge}");
        }
        print!("{}\n{}", "- Chat Settings:".bold(), self.chat_settings);
        print!(
            "{} {}",
            "- Downtime:".bold(),
            self.last_broadcast.time_since()
        );
        Ok(())
    }
}

impl Roles {
    pub fn get_active(&self) -> Vec<String> {
        let mut roles: Vec<String> = Vec::new();
        if self.is_affiliate {
            roles.push(String::from("Affiliate"));
        }
        if self.is_partner {
            roles.push(String::from("Partner"));
        }
        if self.is_staff.unwrap_or(false) {
            roles.push(String::from("Staff"));
        }
        roles
    }
}

impl Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("  {} {}", "- Affiliate:".bold(), yes_no(self.is_affiliate));
        println!("  {} {}", "- Partner:".bold(), yes_no(self.is_partner));
        write!(
            f,
            "{} {}",
            "  - Staff:".bold(),
            yes_no(self.is_staff.unwrap_or_default())
        )
    }
}

impl Badge {
    pub fn color(&self) -> String {
        match self.title.as_ref() {
            "Verified" => self.title.truecolor(196, 77, 255).to_string(),
            "Prime Gaming" => self.title.truecolor(38, 139, 255).to_string(),
            "GLHF Pledge" => self.title.white().to_string(),
            "GlitchCon 2020" => self.title.truecolor(242, 179, 255).to_string(),
            "TwitchCon 2020 - Amsterdam" => self.title.truecolor(170, 0, 204).to_string(),
            _ => self.title.white().to_string(),
        }
    }
}

impl Display for Badge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  {} {}", "-".bold(), self.color().bold())
    }
}

impl Display for ChatSettings {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!(
            "  {} {}",
            "- Chat delay:".bold(),
            format::duration_to_hms(self.chat_delay_ms).bold().green()
        );
        println!(
            "  {} {}",
            "- Followers only duration:".bold(),
            format::duration_to_hms(self.followers_only_duration_minutes)
                .bold()
                .green()
        );
        println!(
            "  {} {}",
            "- Slow mode duration:".bold(),
            format::duration_to_hms(self.slow_mode_duration_seconds)
                .bold()
                .green()
        );
        println!("  {} {}", "- Emote only:".bold(), yes_no(self.emote_only));
        println!("  {} {}", "- Sub only:".bold(), yes_no(self.sub_only));
        println!("  {} {}", "- Unique chat:".bold(), yes_no(self.unique_chat));
        println!(
            "  {} {}",
            "- Require verified account:".bold(),
            yes_no(self.require_verified_account)
        );
        println!("  {}", "- Chat rules:".bold());
        for rule in self.chat_rules.iter() {
            println!("    {} {}", "-".bold(), rule.bold().yellow());
        }
        Ok(())
    }
}

impl LastBroadcast {
    pub fn time_since(&self) -> String {
        let now = Utc::now();
        let d = now.signed_duration_since(self.started_at);
        format::duration_to_hms(d)
    }
}

fn yes_no(b: bool) -> String {
    if b {
        "yes".bold().green().to_string()
    } else {
        "no".bold().red().to_string()
    }
}

fn deserialize_uid<'de, D>(data: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(data).unwrap();
    Ok(s.parse::<u32>().unwrap_or(0))
}
