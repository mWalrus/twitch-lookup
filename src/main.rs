mod cli;
mod config;
mod format;
mod helix;
mod tmi;

use anyhow::Result;
use clap::Parser;
use cli::{Action, Args};
use colored::*;
use config::Config;
use helix::HelixClient;
use tmi::Chat;

// NOTE: https://gist.github.com/Chronophylos/512675897009f26472dd3cfc6b6744cb
//       https://github.com/Supinic/supibot-package-manager/blob/master/commands/subage/index.js

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read()?;
    let args = Args::parse();

    match args.action {
        Action::Chat {
            channel,
            broadcaster,
            mods,
            vips,
            count,
            regular: _,
            present,
        } => {
            let chat = Chat::fetch(&channel).await?;
            if count {
                println!("{} {}", "Chatter count:".bold(), chat.chatter_count());
                return Ok(());
            }

            let chatters = chat.chatters();

            if let Some(user) = present {
                if chatters.is_present(user) {
                    println!("{}", "That user is in chat".bold().bright_green());
                } else {
                    println!("{}", "That user is not in chat".bold().bright_red());
                }
                return Ok(());
            }

            if broadcaster {
                let message = if chatters.broadcaster().is_empty() {
                    "Broadcaster is not in chat".bold().bright_red()
                } else {
                    "Broadcaster is in chat".bold().bright_green()
                };

                println!("{message}")
            }

            let items = if mods {
                chatters.moderators()
            } else if vips {
                chatters.vips()
            } else {
                // regular
                chatters.viewers()
            };

            for (i, item) in items.iter().enumerate() {
                let out = format!(
                    "{}{}\t{}",
                    (i + 1).to_string().bold().blue(),
                    ".".bold().blue(),
                    item.bold()
                );
                println!("{out}");
            }
        }
        Action::Command { bot, cmd_name } => {
            if bot == "supi" || bot == "supibot" {
                let uri = format!("https://supinic.com/bot/command/detail/{cmd_name}");
                webbrowser::open(&uri)?;
            }
            // TODO: add more bots
        }
        Action::User {
            login,
            broadcaster_type,
            uid,
            created,
            name,
            views,
            type_of_user,
            profile_image,
        } => {
            let client = HelixClient::new(config);
            let user = client.get_user(&login).await?;
            let display_name = user.display_name();
            let result = if broadcaster_type {
                format!(
                    "{display_name}'s broadcaster_type is: {}",
                    user.broadcaster_type().blue()
                )
            } else if uid {
                format!("{display_name}'s user id is: {}", user.uid().magenta())
            } else if created {
                format!(
                    "{display_name}'s account was created on {}",
                    user.created_at().green()
                )
            } else if name {
                format!("{login}'s display name is {display_name}")
            } else if views {
                format!("{display_name} has {} views", user.view_count().magenta())
            } else if type_of_user {
                format!("{display_name}'s user type is: {}", user.user_type().blue())
            } else if profile_image {
                format!(
                    "{display_name}'s profile image: {}",
                    user.profile_image().blue()
                )
            } else {
                let broadcaster_type = format!(
                    "{} {}",
                    "- Broadcaster type:".bold(),
                    user.broadcaster_type().blue().bold()
                );
                let uid = format!("{} {}", "- UID:".bold(), user.uid().magenta().bold());
                let created = format!(
                    "{} {}",
                    "- Created at:".bold(),
                    user.created_at().green().bold()
                );
                let views = format!(
                    "{} {}",
                    "- Views".bold(),
                    user.view_count().magenta().bold()
                );
                let user_type = format!(
                    "{} {}",
                    "- User type:".bold(),
                    user.user_type().blue().bold()
                );
                let profile_image = format!(
                    "{} {}",
                    "- Profile image:".bold(),
                    user.profile_image().blue().bold()
                );
                let header = format!("{display_name}'s profile info");
                format!(
                    "{}\n{}\n{}\n{}\n{}\n{}\n{}",
                    header.bold(),
                    uid,
                    created,
                    views,
                    broadcaster_type,
                    user_type,
                    profile_image,
                )
            };
            println!("{}", result.bold());
        }
        _ => {}
    }
    Ok(())
}
