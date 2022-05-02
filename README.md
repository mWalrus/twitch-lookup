# Twitch Lookup
A general twitch chat and user lookup tool.

# Why?
This tool performs many of the same tasks as bots do in different channels. The motivation for creating this tool (apart from being a fun project) was to not have to rely on bot uptimes and also the fact that sometimes you wanna do a lil stalking without typing it in the public chat.

# Prerequisites
- rust + cargo
- cargo-make (`cargo install cargo-make`)

# Installation
1. Clone repository: `git clone https://gitlab.com/mWalrus/twitch-lookup.git`
2. Enter directory: `cd twitch-lookup`
3. Make: `cargo make install`

A new binary called `tl` will be built and installed on your system.

# Help
Run `tl help` for information about commands. You can also do `tl help <subcommand>` for a more detailed view over the given subcommand.
