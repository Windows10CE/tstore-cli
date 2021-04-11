#[macro_use]
extern crate clap;
use clap::{app_from_crate, Arg, AppSettings};

mod commands;
mod models;
use commands::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = app_from_crate!().setting(AppSettings::SubcommandRequiredElseHelp);

    app = app
    .arg(
        Arg::with_name("token")
            .help("Service account token to use")
            .short("t")
            .long("token")
            .env("TSTORE_TOKEN")
            .hide_env_values(true)
            .takes_value(true)
            .global(true),
    )
    .arg(
        Arg::with_name("subdomain")
            .help("Sets the Thunderstore subdomain to use when making requests. Defaults to the RoR2 community (as it doesn't use a subdomain)")
            .short("d")
            .long("domain")
            .takes_value(true)
            .global(true)
    );

    app = app
        .subcommand(publish::create_subcommand())
        .subcommand(info::create_subcommand())
        .subcommand(download::create_subcommand());

    let matches = app.get_matches();

    let url: String;

    if let Some(subdomain) = matches.value_of("subdomain") {
        url = format!("https://{}.thunderstore.io/api", subdomain);
    } else {
        url = String::from("https://thunderstore.io/api");
    }

    if let Some(matches) = matches.subcommand_matches("publish") {
        publish::process_command(matches, url)?;
    } else if let Some(matches) = matches.subcommand_matches("info") {
        info::proccess_command(matches, url)?;
    } else if let Some(matches) = matches.subcommand_matches("download") {
        download::process_command(matches, url)?;
    }

    Ok(())
}
