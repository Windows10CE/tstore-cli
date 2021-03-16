#[macro_use]
extern crate clap;
use clap::app_from_crate;

mod commands;
use commands::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = app_from_crate!();

    app = app.subcommand(publish::create_subcommand());

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("publish") {
        publish::process_command(matches)?;
    }

    Ok(())
}
