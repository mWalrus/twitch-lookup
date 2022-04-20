mod cli;
mod config;
mod decapi;
mod format;
mod gql;
mod helix;
mod leppunen;
mod tmi;

use anyhow::Result;
use clap::Parser;
use cli::{Action, Args, UserAction};
use colored::Colorize;
use config::Config;
use helix::HelixClient;
use leppunen::user::{CompactUser, VerboseUser};
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
        Action::User(user_action) => match user_action {
            UserAction::Compact { user } => {
                let user: Box<dyn CompactUser> = Box::new(leppunen::API::user(&user).await?);
                user.print()?;
            }
            UserAction::Verbose { user } => {
                let user: Box<dyn VerboseUser> = Box::new(leppunen::API::user(&user).await?);
                user.print()?;
            }
            UserAction::Uid { user } => {
                let user = leppunen::API::user(&user).await?;
                println!(
                    "{}{} {}",
                    user.display_name_colored().bold(),
                    "'s user id:".bold(),
                    user.uid().to_string().bold().magenta()
                );
            }
            UserAction::Bc { user } => {
                let user = leppunen::API::user(&user).await?;
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
                let user = leppunen::API::user(&user).await?;
                println!("{}", user.display_name_colored().bold());
            }
            UserAction::Uf { user } => {
                let user = leppunen::API::user(&user).await?;
                println!(
                    "{} {} {} {}",
                    user.display_name_colored().bold(),
                    "is following".bold(),
                    user.following().bold(),
                    "people".bold()
                );
            }
            UserAction::Fu { user } => {
                let user = leppunen::API::user(&user).await?;
                println!(
                    "{} {} {} {}",
                    user.display_name_colored().bold(),
                    "has".bold(),
                    user.followers().bold(),
                    "followers".bold()
                );
            }
            UserAction::Cv { user } => {
                let user = leppunen::API::user(&user).await?;
                println!(
                    "{} {} {} {}",
                    user.display_name_colored().bold(),
                    "has".bold(),
                    user.channel_views().bold(),
                    "channel views".bold()
                );
            }
            UserAction::Cc { user } => {
                let user = leppunen::API::user(&user).await?;
                println!("{}", user.chat_color().bold());
            }
            UserAction::Pfp { user } => {
                let user = leppunen::API::user(&user).await?;
                println!(
                    "{}{} {}",
                    user.display_name_colored().bold(),
                    "'s profile image:".bold(),
                    user.logo.bold().blue()
                );
            }
            UserAction::Bot { user } => {
                let user = leppunen::API::user(&user).await?;
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
                let user = leppunen::API::user(&user).await?;
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
                let user = leppunen::API::user(&user).await?;
                println!(
                    "{}{} {}",
                    user.display_name_colored().bold(),
                    "'s emote prefix:".bold(),
                    user.emote_prefix.bold()
                );
            }
            UserAction::Roles { user } => {
                let user = leppunen::API::user(&user).await?;
                println!("{}", "Roles:".bold());
                for role in user.roles.get_active() {
                    println!("{} {}", "-".bold(), role.bold().green());
                }
            }
            UserAction::Badges { user } => {
                let user = leppunen::API::user(&user).await?;
                println!("{}", "Badges:".bold());
                for badge in user.badges.iter() {
                    println!("{} {}", "-".bold(), badge.color().bold());
                }
            }
            UserAction::Cs { user } => {
                let user = leppunen::API::user(&user).await?;
                print!(
                    "{}{}\n{}",
                    user.display_name_colored().bold(),
                    "'s chat settings:".bold(),
                    user.chat_settings
                );
            }
            UserAction::Dt { user } => {
                let user = leppunen::API::user(&user).await?;
                println!(
                    "{} {} {}",
                    user.display_name_colored().bold(),
                    "has been offline for".bold(),
                    user.last_broadcast.time_since().bold()
                );
            }
            UserAction::Link { user } => {
                let url = format!("https://twitch.tv/{user}");
                println!("{}", url.bold().blue());
            }
        },
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
