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
                .help("ZIP file to publish")
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
}

fn file_exists(filename: String) -> Result<(), String> {
    if std::fs::File::open(filename).is_err() {
        return Err(String::from("ZIP file doesn't exist"));
    }
    Ok(())
}

pub fn process_command(
    matches: &ArgMatches,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let options = PublishOptions::from_matches_and_file(
        matches,
        matches.value_of("config").unwrap_or("publish.toml"),
    )?;

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

    let res = Client::new()
        .post(format!("{}/experimental/package/upload/", url))
        .bearer_auth(options.token)
        .multipart(form)
        .send()?;

    if res.status() != StatusCode::OK {
        return Err(From::from(res.text()?));
    }

    println!("Package uploaded successfully!");
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::Value::from(res.text()?))?
    );

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
                author_name: String::from(
                    matches
                        .value_of("author")
                        .or_else(|| backup_options["team_name"].as_str())
                        .ok_or("")?,
                ),
                categories: matches
                    .values_of_lossy("categories")
                    .or_else(|| {
                        backup_options["categories"].as_array().map(|x| {
                            x.iter()
                                .map(|y| String::from(y.as_str().unwrap()))
                                .collect()
                        })
                    })
                    .unwrap_or(vec![]),
                communities: matches
                    .values_of_lossy("communities")
                    .or_else(|| {
                        backup_options["communities"].as_array().map(|x| {
                            x.iter()
                                .map(|y| String::from(y.as_str().unwrap()))
                                .collect()
                        })
                    })
                    .ok_or("")?,
                nsfw: matches.is_present("nsfw")
                    || backup_options["nsfw"].as_bool().unwrap_or(false),
                zip: String::from(
                    matches
                        .value_of("zip")
                        .or_else(|| {
                            if let Some(other_zip) = backup_options["zip"].as_str() {
                                if file_exists(String::from(other_zip)).is_ok() {
                                    return Some(other_zip);
                                }
                            }
                            None
                        })
                        .ok_or("")?,
                ),
                token: String::from(
                    matches
                        .value_of("token")
                        .or_else(|| backup_options["token"].as_str())
                        .ok_or("This command requires a service account token! Use -t, --token, the the TSTORE_TOKEN env var to set the token.")?,
                ),
            });
        } else {
            return Ok(PublishOptions {
                author_name: String::from(matches.value_of("author").ok_or("")?),
                categories: matches.values_of_lossy("categories").unwrap_or(vec![]),
                communities: matches.values_of_lossy("").ok_or("")?,
                nsfw: matches.is_present("nsfw"),
                zip: String::from(matches.value_of("zip").ok_or("")?),
                token: String::from(matches.value_of("token").ok_or("This command requires a service account token! Use -t, --token, the the TSTORE_TOKEN env var to set the token.")?),
            });
        }
    }
}
