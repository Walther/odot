use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

extern crate clap;
use clap::{App, Arg};
extern crate uuid;
use uuid::Uuid;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
use chrono::prelude::*;

extern crate dirs;

#[derive(Serialize, Deserialize)]
pub struct Odot {
    message: String,
    tags: Vec<String>,
    timestamp: DateTime<Utc>,
    uuid: Uuid,
}

fn main() {
    let matches = App::new("odot")
        .version("0.1")
        .about("Reverse todo: Write down things that you've done.")
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .value_name("MESSAGE")
                .help("Sets the odot message")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tags")
                .short("t")
                .long("tags")
                .value_name("TAGS")
                .help("Sets the odot tags")
                .takes_value(true),
        )
        .get_matches();

    let mut message = String::new();
    match matches.value_of("message") {
        None => {
            io::stdin()
                .read_line(&mut message)
                .expect("Error reading message");
            message = message.trim().to_string();
        }
        Some(msg) => message = msg.to_string(),
    }

    let mut tag_string = String::new();
    let tags: Vec<String>;
    if let Some(tagparams) = matches.value_of("tags") {
        tag_string = tagparams.to_string();
    };

    tags = tag_string
        .split(',')
        .map(|tag| tag.trim().to_string())
        .collect();

    let timestamp: DateTime<Utc> = Utc::now();
    let uuid = Uuid::new_v4();
    let odot = Odot {
        message,
        tags,
        timestamp,
        uuid,
    };

    let data = json!(odot);

    let mut data_dir = dirs::home_dir().expect("Error getting the home directory");
    data_dir.push(".odot/data");
    if !data_dir.as_path().exists() {
        fs::create_dir_all(&data_dir).expect("Error creating the data directory");
    }

    let mut filepath = PathBuf::from(data_dir.as_path());
    filepath.push(format!("{}.json", uuid));
    let mut file = File::create(filepath).expect("Error creating a file");
    write!(file, "{}", data).expect("Error writing data to the file");
}
