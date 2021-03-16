#[macro_use]
extern crate clap;
use clap::{app_from_crate, Arg};

mod commands;
use commands::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = app_from_crate!();

    app = app.arg(
        Arg::with_name("token")
            .help("Service account token to use")
            .short("t")
            .long("token")
            .env("TSTORE_TOKEN")
            .takes_value(true)
            .global(true),
    );

    app = app.subcommand(publish::create_subcommand());

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("publish") {
        publish::process_command(matches)?;
    }

    Ok(())
}
