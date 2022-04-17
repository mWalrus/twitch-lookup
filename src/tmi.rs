use anyhow::Result;
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chat {
    chatter_count: u32,
    chatters: Chatters,
}

#[derive(Deserialize)]
pub struct Chatters {
    broadcaster: Vec<String>,
    moderators: Vec<String>,
    vips: Vec<String>,
    viewers: Vec<String>,
}

impl Chat {
    pub async fn fetch(channel: &str) -> Result<Self> {
        let resp = get(format!(
            "https://tmi.twitch.tv/group/user/{}/chatters",
            channel
        ))
        .await?
        .json::<Chat>()
        .await?;
        Ok(resp)
    }

    pub fn chatter_count(&self) -> u32 {
        self.chatter_count
    }

    pub fn chatters(&self) -> &Chatters {
        &self.chatters
    }
}

impl Chatters {
    pub fn broadcaster(&self) -> &str {
        if self.broadcaster.is_empty() {
            return "";
        } else {
            &self.broadcaster[0]
        }
    }

    pub fn moderators(&self) -> &Vec<String> {
        &self.moderators
    }

    pub fn vips(&self) -> &Vec<String> {
        &self.vips
    }

    pub fn viewers(&self) -> &Vec<String> {
        &self.viewers
    }

    pub fn is_present(&self, user: String) -> bool {
        for moderator in &self.moderators {
            if moderator == &user {
                return true;
            }
        }
        for vip in &self.vips {
            if vip == &user {
                return true;
            }
        }
        for viewer in &self.viewers {
            if viewer == &user {
                return true;
            }
        }
        if let Some(br) = self.broadcaster.first() {
            return br == &user;
        }
        false
    }
}
