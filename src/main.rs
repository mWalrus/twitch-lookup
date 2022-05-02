mod cli;
mod config;
mod decapi;
mod deser;
mod format;
mod gql;
mod helix;
mod leppunen;
mod tmi;

use anyhow::Result;
use clap::Parser;
use cli::{Action, Args, ChatAction, UserAction};
use colored::Colorize;
use config::Config;
use helix::HelixClient;
use leppunen::user::{CompactUser, VerboseUser};
use tmi::Chat;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read()?;
    let args = Args::parse();

    match args.action {
        Action::Chat(chat_action) => match chat_action {
            ChatAction::Streamer { channel } => {
                let chat = Chat::fetch(&channel).await?;
                if chat.chatters().broadcaster().is_empty() {
                    println!(
                        "{} {}",
                        &channel.bold().red(),
                        "is not in chat".bold().red()
                    );
                    return Ok(());
                }
                println!(
                    "{} {}",
                    &channel.bold().bright_green(),
                    "is in chat".bold().bright_green()
                );
            }
            ChatAction::Mods { channel } => {
                let chat = Chat::fetch(&channel).await?;
                let mods = chat.chatters().moderators();
                println!(
                    "{} {}{}",
                    "Moderators in".bold(),
                    &channel.bold(),
                    ":".bold()
                );
                for (i, moderator) in mods.iter().enumerate() {
                    println!(
                        "{} {}",
                        (i + 1).to_string().bold().magenta(),
                        moderator.bold()
                    );
                }
            }
            ChatAction::Vips { channel } => {
                let chat = Chat::fetch(&channel).await?;
                let vips = chat.chatters().vips();
                println!("{} {}{}", "VIPs in".bold(), &channel.bold(), ":".bold());
                for (i, vip) in vips.iter().enumerate() {
                    println!("{} {}", (i + 1).to_string().bold().magenta(), &vip.bold());
                }
            }
            ChatAction::Normals { channel } => {
                let chat = Chat::fetch(&channel).await?;
                let normals = chat.chatters().viewers();
                println!(
                    "{} {}{}",
                    "Normal chatters in".bold(),
                    &channel.bold(),
                    ":".bold()
                );
                for (i, normal) in normals.iter().enumerate() {
                    println!(
                        "{} {}",
                        (i + 1).to_string().bold().magenta(),
                        &normal.bold()
                    );
                }
            }
            ChatAction::Count { channel } => {
                let chat = Chat::fetch(&channel).await?;
                let count = chat.chatter_count();
                println!(
                    "{} {} {} {}",
                    "There are currently".bold(),
                    count.to_string().bold().magenta(),
                    "chatters in".bold(),
                    &channel.bold()
                );
            }
            ChatAction::Present { user, channel } => {
                let chat = Chat::fetch(&channel).await?;
                if chat.chatters().is_present(&user) {
                    println!(
                        "{} {} {}{}",
                        &user.bold(),
                        "is currently present in".bold(),
                        &channel.bold(),
                        "'s chat".bold()
                    );
                    return Ok(());
                }
                println!(
                    "{} {} {} {} {}{}",
                    &user.bold(),
                    "is".bold(),
                    "not".bold().red(),
                    "currently in".bold(),
                    &channel.bold(),
                    "'s chat".bold()
                );
            }
        },
        Action::Command { bot, cmd_name } => {
            if bot == "supi" || bot == "supibot" {
                let uri = format!("https://supinic.com/bot/command/detail/{cmd_name}");
                webbrowser::open(&uri)?;
            }
            // TODO: add more bots
        }
        Action::User(user_action) => match user_action {
            UserAction::Compact { user } => {
                let user: Box<dyn CompactUser> = Box::new(leppunen::Api::user(&user).await?);
                user.print()?;
            }
            UserAction::Verbose { user } => {
                let user: Box<dyn VerboseUser> = Box::new(leppunen::Api::user(&user).await?);
                user.print()?;
            }
            UserAction::Bc { user } => {
                let user = leppunen::Api::user(&user).await?;
                if user.banned {
                    println!(
                        "{} {}",
                        user.display_name_colored().bold(),
                        "is banned".bold().red()
                    );
                } else {
                    println!(
                        "{} {}",
                        user.display_name_colored().bold(),
                        "is not banned".bold().green()
                    );
                }
            }
            UserAction::Dn { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!("{}", user.display_name_colored().bold());
            }
            UserAction::Uf { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!(
                    "{} {} {} {}",
                    user.display_name_colored().bold(),
                    "is following".bold(),
                    user.following().bold(),
                    "people".bold()
                );
            }
            UserAction::Fu { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!(
                    "{} {} {} {}",
                    user.display_name_colored().bold(),
                    "has".bold(),
                    user.followers().bold(),
                    "followers".bold()
                );
            }
            UserAction::Cv { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!(
                    "{} {} {} {}",
                    user.display_name_colored().bold(),
                    "has".bold(),
                    user.channel_views().bold(),
                    "channel views".bold()
                );
            }
            UserAction::Cc { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!("{}", user.chat_color().bold());
            }
            UserAction::Pfp { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!(
                    "{}{} {}",
                    user.display_name_colored().bold(),
                    "'s profile image:".bold(),
                    user.logo.bold().blue()
                );
            }
            UserAction::Bot { user } => {
                let user = leppunen::Api::user(&user).await?;
                if user.verified_bot {
                    println!(
                        "{} {}",
                        user.display_name_colored().bold(),
                        "is a verified bot".bold().green()
                    );
                } else {
                    println!(
                        "{} {}",
                        user.display_name_colored().bold(),
                        "is not a bot".bold().red()
                    );
                }
            }
            UserAction::Cd { user } => {
                let user = leppunen::Api::user(&user).await?;
                // TODO: time since account creation
                println!(
                    "{} {} {}",
                    user.display_name_colored().bold(),
                    "was created on".bold(),
                    user.created_at
                        .date()
                        .naive_utc()
                        .to_string()
                        .bold()
                        .green()
                );
            }
            UserAction::Ep { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!(
                    "{}{} {}",
                    user.display_name_colored().bold(),
                    "'s emote prefix:".bold(),
                    user.emote_prefix.bold()
                );
            }
            UserAction::Roles { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!("{}", "Roles:".bold());
                for role in user.roles.get_active() {
                    println!("{} {}", "-".bold(), role.bold().green());
                }
            }
            UserAction::Badges { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!("{}", "Badges:".bold());
                for badge in user.badges.iter() {
                    println!("{} {}", "-".bold(), badge.color().bold());
                }
            }
            UserAction::Cs { user } => {
                let user = leppunen::Api::user(&user).await?;
                print!(
                    "{}{}\n{}",
                    user.display_name_colored().bold(),
                    "'s chat settings:".bold(),
                    user.chat_settings
                );
            }
            UserAction::Dt { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!(
                    "{} {} {}",
                    user.display_name_colored().bold(),
                    "has been offline for".bold(),
                    user.last_broadcast.time_since().bold()
                );
            }
            UserAction::Uid { user } => {
                let user = leppunen::Api::user(&user).await?;
                println!(
                    "{}{} {}",
                    user.display_name_colored().bold(),
                    "'s user ID is:".bold(),
                    user.uid().to_string().bold().magenta()
                );
            }
            UserAction::Link { user } => {
                let url = format!("https://twitch.tv/{user}");
                println!("{}", url.bold().blue());
            }
        },
        Action::Logs { user, channel } => {
            if let Some(channel) = channel {
                if leppunen::Api::is_valid_logs_query(&user, &channel).await {
                    let url = format!("https://logs.ivr.fi/?channel={channel}&username={user}");
                    webbrowser::open(&url)?;
                } else {
                    println!(
                        "{} {} {}",
                        "That user or channel could".bold(),
                        "not".bold().red(),
                        "be found".bold()
                    );
                }
            } else {
                let login = config.login();
                if leppunen::Api::is_valid_logs_query(&login, &user).await {
                    let url = format!("https://logs.ivr.fi/?channel={user}&username={login}");
                    webbrowser::open(&url)?;
                } else {
                    println!(
                        "{} {} {}",
                        "That user or channel could".bold(),
                        "not".bold().red(),
                        "be found".bold()
                    );
                }
            };
        }
        Action::Fa { user, channel } => {
            let (user, target) = if let Some(c) = channel {
                (user, c)
            } else {
                (config.login().to_string(), user)
            };
            let fa = decapi::follow_age(&user, &target).await?;
            let output = if fa.contains("does not follow") {
                format!(
                    "{} does {} follow {}",
                    user.blue(),
                    "not".red(),
                    target.blue()
                )
            } else {
                format!(
                    "{} has followed {} for {}",
                    user.blue(),
                    target.blue(),
                    fa.green()
                )
            };
            println!("{}", output.bold());
        }
        Action::Title { channel } => {
            println!(
                "{}\n{}{}",
                decapi::title(&channel).await?.bold(),
                "https://twitch.tv/".blue().bold(),
                channel.blue().bold()
            );
        }
        Action::Live { channel } => {
            if let Some(view_count) = decapi::is_live(&channel).await {
                println!(
                    "{}\n{}{}",
                    format!("{channel} is live with {} viewer(s)", view_count.magenta()).bold(),
                    "https://twitch.tv/".bold().blue(),
                    channel.bold().blue()
                )
            } else {
                println!(
                    "{} {}",
                    channel.blue().bold(),
                    "is currently offline".bold()
                )
            }
        }
        Action::Subbed { user, channel } => {
            // FIXME: use gql to be able to check other users
            let (user, target) = if let Some(c) = channel {
                (user, c)
            } else {
                (config.login().to_string(), user)
            };
            let hx_cli = HelixClient::new(config);
            let sub_status = hx_cli.subscription_status(&user, &target).await?;
            println!("{}", sub_status.bold())
        }
        Action::Vods { channel, amount } => {
            let client = HelixClient::new(config);
            let vods = client.get_vods(&channel, amount).await;
            if let Some(vods) = vods {
                for (i, vod) in vods.iter().enumerate() {
                    println!("{} {}\n{vod}", "Vod".bold(), (i + 1).to_string().bold());
                }
            }
        }
        Action::Ll => {
            let user_id = config.user_id();
            let client = HelixClient::new(config);
            let mut channels = client
                .get_live_followed_channels(user_id)
                .await
                .unwrap_or_default();
            channels.reverse();
            for channel in channels {
                println!("{channel}");
            }
        }
    }
    Ok(())
}
