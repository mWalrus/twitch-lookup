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
use tmi::Chat;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::read();
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
        _ => {}
    }
    Ok(())
}
