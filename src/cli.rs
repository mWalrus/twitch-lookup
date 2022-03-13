use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about = "A simple twitch account lookup tool", long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
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
        // TODO: maybe add an option to get link to users logs in a given channel.
    },
    Chat {
        #[clap(required(true))]
        channel: String,
        #[clap(short, long, required_unless_present_any(["vips", "count", "present"]), help = "Get the moderators currently in the given chat.")]
        mods: bool,
        #[clap(short, long, required_unless_present_any(["mods", "count", "present"]), help = "Get all VIPs currently in the given chat.")]
        vips: bool,
        #[clap(short, long, required_unless_present_any(["mods", "vips", "present"]), help = "Get the total number of chatters currently in the given chat.")]
        count: bool,
        #[clap(short, long, required_unless_present_any(["mods", "vips", "count"]), help = "Check whether a given user is present in the given chat.")]
        present: Option<String>,
    },
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
    Live,
}
