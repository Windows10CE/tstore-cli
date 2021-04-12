use crate::models::package_info::PackageInfo;
use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::{
    blocking::{multipart, Client},
    StatusCode,
};
use serde_json::json;
use toml::Value;

pub fn create_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("publish")
        .about("Publish a ZIP to thunderstore")
        .arg(
            Arg::with_name("zip")
                .help("ZIP file to publish\nCheck the documentation at https://github.com/Windows10CE/tstore-cli for additional notes on usage.")
                .takes_value(true)
                .validator(file_exists),
        )
        .arg(
            Arg::with_name("author")
                .help("Name of team to publish to")
                .short("a")
                .long("author")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("categories")
                .help("Categories that the package should be in")
                .long("categories")
                .takes_value(true)
                .use_delimiter(true),
        )
        .arg(
            Arg::with_name("communities")
                .help("Communities to publish to")
                .short("c")
                .long("communities")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("nsfw")
                .help("Package contains nsfw contents")
                .long("nsfw"),
        )
        .arg(
            Arg::with_name("config")
                .help("The config file to read from.")
                .long("config")
                .default_value("publish.toml")
        )
}

fn file_exists(filename: String) -> Result<(), String> {
    if std::fs::File::open(filename).is_err() {
        return Err("ZIP file doesn't exist".to_string());
    }
    Ok(())
}

pub fn process_command(
    matches: &ArgMatches,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let options =
        PublishOptions::from_matches_and_file(matches, matches.value_of("config").unwrap())?;

    let metadata_json = json!({
        "author_name": options.author_name,
        "categories": options.categories,
        "communities": options.communities,
        "has_nsfw_content": options.nsfw,
    })
    .to_string();

    let form = multipart::Form::new()
        .text("metadata", metadata_json)
        .file("file", options.zip)
        .unwrap();

    let req = Client::new()
        .post(format!("{}/experimental/package/upload/", url))
        .bearer_auth(options.token)
        .multipart(form);

    println!("Uploading package...");
    let res = req.send()?;

    if res.status() != StatusCode::OK {
        return Err(From::from(res.text()?));
    }

    let res_json = res.text()?.parse::<serde_json::Value>()?;
    let new_info = PackageInfo::from_author_and_name(
        res_json["namespace"].as_str().unwrap(),
        res_json["name"].as_str().unwrap(),
        url.as_str(),
    )?;
    println!("Package uploaded successfully! {}", new_info.package_url);

    Ok(())
}

struct PublishOptions {
    pub author_name: String,
    pub categories: Vec<String>,
    pub communities: Vec<String>,
    pub nsfw: bool,
    pub zip: String,
    pub token: String,
}

impl PublishOptions {
    fn from_matches_and_file(
        matches: &ArgMatches,
        backup_file_path: &str,
    ) -> Result<PublishOptions, Box<dyn std::error::Error>> {
        if let Ok(backup) = std::fs::read_to_string(backup_file_path) {
            let backup_options = backup.parse::<Value>()?;
            return Ok(PublishOptions {
                author_name: matches
                    .value_of("author")
                    .or_else(|| backup_options.get("author").map(|x| x.as_str().unwrap()))
                    .ok_or("This command requires an author name to upload under. Check help using tstore-cli publish --help")?
                    .to_string(),
                categories: matches
                    .values_of_lossy("categories")
                    .or_else(|| {
                        if let Some(backup_option) = backup_options.get("categories") {
                            return backup_option.as_array().map(|x| {
                                x.iter()
                                    .map(|y| y.as_str().unwrap().to_string())
                                    .collect()
                            });
                        }
                        None
                    })
                    .unwrap_or(vec![]),
                communities: matches
                    .values_of_lossy("communities")
                    .or_else(|| {
                        if let Some(backup_option) = backup_options.get("communities") {
                            return backup_option.as_array().map(|x| {
                                x.iter()
                                    .map(|y| y.as_str().unwrap().to_string())
                                    .collect()
                            });
                        }
                        None
                    })
                    .ok_or("This command requires you to specify one or more communities to upload to. Check help using tstore-cli publish --help")?,
                nsfw: matches.is_present("nsfw")
                    || backup_options.get("nsfw").map(|x| x.as_bool().unwrap()).unwrap_or(false),
                zip: matches
                    .value_of("zip")
                    .or_else(|| {
                        if let Some(other_zip) = backup_options.get("zip").map(|x| x.as_str().unwrap()) {
                            if file_exists(other_zip.to_string()).is_ok() {
                                return Some(other_zip);
                            }
                        }
                        None
                    })
                    .ok_or("This command requires a path to a ZIP file to upload. Check help using tstore-cli publish --help")?
                    .to_string(),
                token: matches
                    .value_of("token")
                    .or_else(|| backup_options.get("token").map(|x| x.as_str().unwrap()))
                    .ok_or("This command requires a service account token! Check help using tstore-cli publish --help")?
                    .to_string(),
            });
        } else {
            return Ok(PublishOptions {
                author_name: matches.value_of("author").ok_or("This command requires an author name to upload under. Check help using tstore-cli publish --help")?.to_string(),
                categories: matches.values_of_lossy("categories").unwrap_or(vec![]),
                communities: matches.values_of_lossy("communities").ok_or("This command requires you to specify one or more communities to upload to. Check help using tstore-cli publish --help")?,
                nsfw: matches.is_present("nsfw"),
                zip: matches.value_of("zip").ok_or("This command requires a path to a ZIP file to upload. Check help using tstore-cli publish --help")?.to_string(),
                token: matches.value_of("token").ok_or("This command requires a service account token! Check help using tstore-cli publish --help")?.to_string(),
            });
        }
    }
}
