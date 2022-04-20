use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about = "A simple twitch account lookup tool", long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

// FIXME: overhaul this subcommand
//        each field should be converted to subcommands.
//        example: `tl user m0xyy roles -p` to check if m0xyy is partner
//        example: `tl user m0xyy bc` to ban check m0xyy
//        u get the idea.
#[non_exhaustive]
#[derive(Subcommand, Debug)]
pub enum Action {
    #[clap(about = "Get general user account information")]
    User {
        #[clap(required(true))]
        login: String,
        #[clap(short, long, help = "Get the broadcaster type of a given user.")]
        broadcaster_type: bool,
        #[clap(short, long, help = "Get the UID of a given user.")]
        uid: bool,
        #[clap(short, long, help = "Get the date a given user was created.")]
        created: bool,
        #[clap(short, long, help = "Get the name of a given user.")]
        name: bool,
        #[clap(short, long, help = "Get the views of a given user.")]
        views: bool,
        #[clap(short, long, help = "Get the type of a given user.")]
        type_of_user: bool,
        #[clap(short, long, help = "Get the user profile picture link.")]
        profile_image: bool,
        #[clap(short, long, help = "Get the user profile link.")]
        link: bool,
    },
    #[clap(about = "Get chat information for a given account")]
    Chat {
        #[clap(required(true))]
        channel: String,
        #[clap(short, long, required_unless_present_any(["vips", "mods", "regular", "count", "present"]), help = "Get the moderators currently in the given chat.")]
        broadcaster: bool,
        #[clap(short, long, required_unless_present_any(["vips", "broadcaster", "regular", "count", "present"]), help = "Get the moderators currently in the given chat.")]
        mods: bool,
        #[clap(short, long, required_unless_present_any(["mods", "broadcaster", "count", "present", "regular"]), help = "Get all VIPs currently in the given chat.")]
        vips: bool,
        #[clap(short, long, required_unless_present_any(["mods", "broadcaster", "regular", "vips", "present"]), help = "Get the total number of chatters currently in the given chat.")]
        count: bool,
        #[clap(short, long, required_unless_present_any(["vips", "broadcaster", "mods", "count", "present"]), help = "Get the moderators currently in the given chat.")]
        regular: bool,
        #[clap(short, long, required_unless_present_any(["mods", "broadcaster", "regular", "vips", "count"]), help = "Check whether a given user is present in the given chat.")]
        present: Option<String>,
    },
    #[clap(about = "Follow information (doesn't do anything at the moment)")]
    Follow {
        #[clap(required(true))]
        user: String,
        #[clap(long, required_unless_present_any(["following", "follows"]), help = "Get the followers of the given user.")]
        followers: bool,
        #[clap(long, required_unless_present_any(["followers", "follows"]), help = "Get the accounts the given user is following.")]
        following: bool,
        #[clap(long, required_unless_present_any(["followers", "following"]), help = "Check the follow relationship between two users.")]
        follows: Option<String>,
    },
    #[clap(about = "Check if channel is live and how many viewers they have at the moment")]
    Live { channel: String },
    #[clap(about = "Open up a given bot command in your default browser")]
    Command { bot: String, cmd_name: String },
    #[clap(about = "Open up a given users logs in a given channel in your default browser")]
    Logs { user: String, channel: String },
    #[clap(about = "Check how long a user has followed a given channel")]
    Fa { user: String, channel: String },
    #[clap(about = "Get the stream title from a given channel")]
    Title { channel: String },
    #[clap(about = "Check your own subscription status to a given channel")]
    Subbed { user: String, channel: String },
    #[clap(about = "Get VOD(s) from a given channel")]
    Vods { channel: String, amount: Option<u8> },
    #[clap(about = "Get the current downtime for a given channel")]
    Dt { channel: String },
}
