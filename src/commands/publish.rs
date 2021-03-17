use clap::{App, Arg, ArgMatches, SubCommand};
use reqwest::{
    blocking::{multipart, Client},
    StatusCode,
};
use serde_json::json;

pub fn create_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("publish")
        .about("Publish a ZIP to thunderstore")
        .arg(
            Arg::with_name("zip")
                .help("ZIP file to publish")
                .required(true)
                .takes_value(true)
                .validator(file_exists),
        )
        .arg(
            Arg::with_name("author")
                .help("Name of team to publish to")
                .short("a")
                .long("author")
                .required(true)
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
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("nsfw")
                .help("Package contains nsfw contents")
                .long("nsfw")
                .required(false),
        )
}

pub fn process_command(
    matches: &ArgMatches,
    url: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let metadata_json = json!({
        "author_name": matches.value_of("author").unwrap(),
        "categories": matches.values_of_lossy("categories").unwrap_or(vec![]),
        "communities": matches.values_of_lossy("communities").unwrap(),
        "has_nsfw_content": matches.is_present("nsfw")
    })
    .to_string();

    let form = multipart::Form::new()
        .text("metadata", metadata_json)
        .file("file", matches.value_of("zip").unwrap())
        .unwrap();

    let res = Client::new()
        .post(format!("{}/experimental/package/upload/", url))
        .bearer_auth(matches.value_of("token").ok_or("This command requires a service account token! Use -t, --token, the the TSTORE_TOKEN env var to set the token.")?)
        .multipart(form)
        .send()?;

    if res.status() != StatusCode::OK {
        return Err(From::from(res.text()?));
    }

    println!("Package uploaded successfully!");

    Ok(())
}

fn file_exists(filename: String) -> Result<(), String> {
    if std::fs::File::open(filename).is_err() {
        return Err("ZIP file doesn't exist".to_string());
    }
    Ok(())
}
