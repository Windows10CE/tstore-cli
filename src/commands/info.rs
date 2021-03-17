use clap::{App, Arg, ArgMatches, SubCommand};
use package_info::PackageInfo;
use reqwest::{blocking::Client};

use crate::models::package_info;

pub fn create_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("info")
        .about("Displays info about a package")
        .arg(
            Arg::with_name("author")
                .help("Name of the author/team of the package")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("package_name")
                .help("Name of the package")
                .takes_value(true)
                .required(true)
                .index(2),
        )
}

pub fn proccess_command(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let res = Client::new().get(format!(
        "https://beta.thunderstore.io/api/experimental/package/{}/{}/",
        matches.value_of("author").unwrap(),
        matches.value_of("package_name").unwrap()
    ))
    .send()?;

    let json: PackageInfo = serde_json::from_str(res.text()?.as_str())?;

    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
