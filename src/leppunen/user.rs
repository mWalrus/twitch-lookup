use crate::format;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use colored::Colorize;
use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub banned: bool,
    pub display_name: String,
    pub bio: String,
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

fn deserialize_millis<'de, D>(data: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis: i64 = Deserialize::deserialize(data).unwrap_or(0);
    Ok(Duration::milliseconds(millis))
}

fn deserialize_seconds<'de, D>(data: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds: i64 = Deserialize::deserialize(data).unwrap_or(0);
    Ok(Duration::seconds(seconds))
}

fn deserialize_minutes<'de, D>(data: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let minutes: i64 = Deserialize::deserialize(data).unwrap_or(0);
    Ok(Duration::minutes(minutes))
}

fn deserialize_date_time<'de, D, S>(data: D) -> Result<S, D::Error>
where
    D: Deserializer<'de>,
    S: FromStr,
    S::Err: Display,
{
    let s: String = Deserialize::deserialize(data)?;
    S::from_str(&s).map_err(de::Error::custom)
}

impl Display for User {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = format::hex_to_rgb(&self.chat_color);
        let follows = format::readable_number(self.follows.into());
        let followers = format::readable_number(self.followers);
        let channel_views = format::readable_number(self.channel_views);

        println!(
            "{}{}",
            self.display_name.bold().truecolor(r, g, b),
            "'s profile information:".bold()
        );
        println!("{} {}", "- Banned:".bold(), yes_no(self.banned));
        println!("{} {}", "- Bio:".bold(), self.bio.bold());
        println!("{} {}", "- Follows:".bold(), follows.bold().magenta());
        println!("{} {}", "- Followers:".bold(), followers.bold().magenta());
        println!(
            "{} {}",
            "- Channel views:".bold(),
            channel_views.bold().magenta()
        );
        println!(
            "{} {}",
            "- Chat color:".bold(),
            self.chat_color.bold().truecolor(r, g, b)
        );
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

impl Display for Badge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.title.as_ref() {
            "Verified" => "Verified".truecolor(196, 77, 255),
            "Prime Gaming" => "Prime Gaming".truecolor(38, 139, 255),
            "GLHF Pledge" => "GLHF Pledge".white(),
            "GlitchCon 2020" => "GlitchCon 2020".truecolor(242, 179, 255),
            "TwitchCon 2020 - Amsterdam" => "TwitchCon 2020 - Amsterdam".truecolor(170, 0, 204),
            _ => self.title.white(),
        };
        write!(f, "  {} {}", "-".bold(), out.bold())
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
    fn time_since(&self) -> String {
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