use serde::{self, Deserialize};

#[derive(Deserialize, Clone)]
pub struct Sub {
    is_gift: bool,
    tier: String,
    #[serde(rename(deserialize = "gifter_name"))]
    gifter: Option<String>,
}

impl Sub {
    pub fn is_gift(&self) -> bool {
        self.is_gift
    }
    pub fn tier(&self) -> u16 {
        self.tier.parse::<u16>().unwrap() / 1000
    }
    pub fn gifter(&self) -> String {
        self.gifter
            .clone()
            .unwrap_or_else(|| String::from("No one"))
    }
}
