use crate::models::package_info::PackageInfo;
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::blocking::Client;
use std::{fs::File, io::Write};

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
        .arg(Arg::with_name("recurse").short("r").long("recurse"))
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

    if !matches.is_present("recurse") {
        let res = Client::new().get(&info.latest.download_url).send()?;

        println!("{} finished downloading!", &info.name);

        File::create(format!(
            "{}-{}.zip",
            &info.full_name, &info.latest.version_number
        ))?
        .write_all(&res.bytes()?)?;
    } else {
        let mut all_packages: Vec<PackageInfo> = vec![];

        process_info(&mut all_packages, info, url.as_str())?;

        let client = Client::new();

        for package in all_packages {
            let res = client.get(package.latest.download_url).send()?;

            println!("{} finished downloading!", &package.name);

            File::create(format!(
                "{}-{}.zip",
                &package.full_name, &package.latest.version_number
            ))?
            .write_all(&res.bytes()?)?;
        }
    }
    Ok(())
}

fn process_info(
    all_infos: &mut Vec<PackageInfo>,
    package: PackageInfo,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for dependency in &package.latest.dependencies {
        let dep_segments: Vec<&str> = dependency.split('-').collect();
        let dependency_info =
            PackageInfo::from_author_and_name(dep_segments[0], dep_segments[1], url)?;
        process_info(all_infos, dependency_info, url)?;
    }
    if !all_infos.iter().any(|x| x.full_name == package.full_name) {
        all_infos.push(package);
    }
    Ok(())
}
