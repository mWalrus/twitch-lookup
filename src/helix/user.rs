use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserData {
    pub data: Vec<User>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    broadcaster_type: String,
    display_name: String,
    #[serde(rename(deserialize = "id"))]
    uid: String,
    #[serde(rename(deserialize = "profile_image_url"))]
    profile_image: Option<String>,
    #[serde(rename(deserialize = "type"))]
    user_type: String,
    created_at: String,
    view_count: u32,
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
        self.profile_image
            .clone()
            .unwrap_or(String::from("Not found"))
    }

    pub fn user_type(&self) -> String {
        self.user_type.to_string()
    }

    pub fn created_at(&self) -> String {
        // NOTE: maybe add real datetime handling
        self.created_at[..10].to_string()
    }

    pub fn view_count(&self) -> String {
        // FIXME: make this more readable with commas and stuff
        self.view_count.to_string()
    }
}
