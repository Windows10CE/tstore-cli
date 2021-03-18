use std::{fs::File, io::Write};
use crate::models::package_info::PackageInfo;
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::blocking::Client;

pub fn create_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("download")
        .alias("dl")
        .about("Downloads a package ZIP from Thunderstore")
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

pub fn process_command(
    matches: &ArgMatches,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let info = PackageInfo::from_author_and_name(
        matches.value_of("author").unwrap(),
        matches.value_of("package_name").unwrap(),
        url.as_str(),
    )?;

    let res = Client::new().get(&info.latest.download_url).send()?;

    let mut file = File::create(format!("{}-{}.zip", &info.full_name, &info.latest.version_number))?;

    print!("{}", res.url());

    file.write_all(&res.bytes()?)?;

    Ok(())
}
