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
        u_type: bool,
    },
    Chat {
        #[clap(required(true))]
        channel: String,
        #[clap(short, long, required_unless_present_any(["vips", "count", "present"]))]
        mods: bool,
        #[clap(short, long, required_unless_present_any(["mods", "count", "present"]))]
        vips: bool,
        #[clap(short, long, required_unless_present_any(["mods", "vips", "present"]))]
        count: bool,
        #[clap(short, long, required_unless_present_any(["mods", "vips", "count"]))]
        present: Option<String>,
    },
    Follow {
        #[clap(required(true))]
        user: String,
        #[clap(long, required_unless_present_any(["following", "follows"]))]
        followers: bool,
        #[clap(long, required_unless_present_any(["followers", "follows"]))]
        following: bool,
        #[clap(long, required_unless_present_any(["followers", "following"]))]
        follows: Option<String>,
    },
    Live,
}
