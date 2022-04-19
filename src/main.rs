mod cli;
mod config;
mod decapi;
mod format;
mod gql;
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
            regular,
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

                println!("{message}");
                return Ok(());
            }

            let items = if mods {
                chatters.moderators()
            } else if vips {
                chatters.vips()
            } else if regular {
                chatters.viewers()
            } else {
                unreachable!()
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
            link,
        } => {
            let client = HelixClient::new(config);
            // Out of name length bounds
            // TODO: names can only contain english letters, numbers and dashes
            //       so we need to check if the given name deviates from these rules.
            if login.len() < 3 || login.len() > 25 {
                println!("{}", "The name provided is not valid!".bold().yellow());
                return Ok(());
            }
            let user = client.get_user(&login).await;
            if user.is_none() {
                println!(
                    "{}",
                    format!("user {} could not be found", login.bold().blue()).bold()
                );
                return Ok(());
            }
            let user = user.unwrap();
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
            } else if link {
                format!(
                    "{display_name}'s profile link: {}{}",
                    "https://twitch.tv/".bold().blue(),
                    login.blue().bold()
                )
            } else {
                println!("{user}");
                return Ok(());
            };
            println!("{}", result.bold());
        }
        Action::Logs { user, channel } => {
            let url = format!("https://logs.ivr.fi/?channel={channel}&username={user}");
            webbrowser::open(&url)?;
        }
        Action::Fa { user, channel } => {
            let fa = decapi::follow_age(&user, &channel).await?;
            let output = format!(
                "{} has followed {} for {}",
                user.blue(),
                channel.blue(),
                fa.green()
            );
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
            let hx_cli = HelixClient::new(config);
            let sub_status = hx_cli.subscription_status(&user, &channel).await?;
            println!("{}", sub_status.bold())
        }
        Action::Ls { channel } => {
            if let Some(url) = decapi::last_stream(&channel).await {
                println!("{} {}", "Last Stream URL:".bold(), url.bold().blue());
            } else {
                println!("{}", format!("{channel} has no vods!").bold());
            }
        }
        Action::Vods { channel, amount } => {
            let client = HelixClient::new(config);
            let vods = client.get_vods(&channel, amount).await;
            if let Some(vods) = vods {
                for (i, vod) in vods.data.iter().enumerate() {
                    println!("{} {}\n{vod}", "Vod".bold(), (i + 1).to_string().bold());
                }
            }
        }
        _ => {}
    }
    Ok(())
}
