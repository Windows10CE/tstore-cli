# tstore-cli
CLI for https://thunderstore.io made in Rust.

## Installation
There are two options to install this program currently, through Cargo or the prebuilt binaries in the GitHub releases.

To install with Cargo, install the Rust toolchain with the instructions from [here](https://doc.rust-lang.org/book/ch01-01-installation.html). Then run `cargo install tstore-cli` to compile and install tstore-cli.

The other option is to grab a pre-compiled binary from [releases](https://github.com/Windows10CE/tstore-cli/releases) or the [Thunderstore page](https://thunderstore.io/package/Windows10CE/tstore_cli) (these are only available for Windows users!)

## Config file for publish command
The config file format uses TOML, with a short example below:
```toml
author = "Windows10CE"
categories = ["Tools"]
communities = ["riskofrain2", "dyson-sphere-program", "valheim", "gtfo", "outward", "cyberpunk2077"]
nsfw = false
zip = "path/to/zip.zip"
token = "service account token"
```
I would highly recommend storing your token outside of your publish.toml, but it's an option nonetheless.

The categories you select will only apply to the community of the subdomain you publish to (for now, categories have planned changes incoming).