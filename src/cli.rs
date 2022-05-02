use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about = "A simple twitch account lookup tool", long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[non_exhaustive]
#[derive(Subcommand, Debug)]
pub enum Action {
    #[clap(subcommand, about = "Get general user account information")]
    User(UserAction),
    #[clap(subcommand, about = "Get chat information for a given account")]
    Chat(ChatAction),
    #[clap(about = "Check if channel is live and how many viewers they have at the moment")]
    Live { channel: String },
    #[clap(about = "Open up a given bot command in your default browser")]
    Command { bot: String, cmd_name: String },
    #[clap(about = "Open up a given users logs in a given channel in your default browser")]
    Logs {
        user: String,
        channel: Option<String>,
    },
    #[clap(about = "Check how long a user has followed a given channel")]
    Fa {
        user: String,
        channel: Option<String>,
    },
    #[clap(about = "Get the stream title from a given channel")]
    Title { channel: String },
    #[clap(about = "Check your own subscription status to a given channel")]
    Subbed {
        user: String,
        channel: Option<String>,
    },
    #[clap(about = "Get VOD(s) from a given channel")]
    Vods { channel: String, amount: Option<u8> },
    #[clap(about = "Get streams from your follow directory")]
    Ll,
}

#[derive(Subcommand, Debug)]
pub enum UserAction {
    #[clap(about = "Compact list of user information")]
    Compact { user: String }, // compact information
    #[clap(about = "Verbose list of user information")]
    Verbose { user: String }, // verbose information
    #[clap(about = "User ID")]
    Uid { user: String },
    #[clap(about = "Ban check")]
    Bc { user: String }, // ban check
    #[clap(about = "Display name")]
    Dn { user: String }, // display name
    #[clap(about = "Following")]
    Uf { user: String }, // user follows
    #[clap(about = "Followers")]
    Fu { user: String }, // follows user
    #[clap(about = "Channel views")]
    Cv { user: String }, // channel views
    #[clap(about = "Chat color")]
    Cc { user: String }, // chat color
    #[clap(about = "Profile picture")]
    Pfp { user: String }, // profile picture
    #[clap(about = "Verified bot check")]
    Bot { user: String }, // verified bot check
    #[clap(about = "Account creation date")]
    Cd { user: String }, // created date
    #[clap(about = "Emote prefix")]
    Ep { user: String }, // emote prefix
    #[clap(about = "Roles")]
    Roles { user: String }, // roles
    #[clap(about = "Badges")]
    Badges { user: String }, // badges
    #[clap(about = "Chat settings")]
    Cs { user: String }, // chat settings
    #[clap(about = "Downtime")]
    Dt { user: String }, // downtime
    #[clap(about = "Twitch URL")]
    Link { user: String },
}

#[derive(Subcommand, Debug)]
pub enum ChatAction {
    Streamer { channel: String },
    Mods { channel: String },
    Vips { channel: String },
    Normals { channel: String },
    Count { channel: String },
    Present { user: String, channel: String },
}
