use clap::{Arg, ArgAction, ArgMatches, Command};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs, io::ErrorKind, process};

const CONFIG_FILENAME: &str = "draft.config.json";

#[derive(Debug, Deserialize, Serialize)]
struct Dotfile {
    name: String,
    out_dir: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    dotfiles: Vec<Dotfile>,
    dot_dir: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            dotfiles: vec![Dotfile {
                name: String::from("filename"),
                out_dir: String::from("/home"),
            }],
            dot_dir: None,
        }
    }
}

pub fn command_dotfiles() -> Command {
    Command::new("dotfiles")
        .about("Create links to your dotfiles using draft.config.json")
        .arg(
            Arg::new("init")
                .long("init")
                .action(ArgAction::SetTrue)
                .help("Create a draft.config.json"),
        )
}

pub fn handle(args: &ArgMatches) {
    let current_path = env::current_dir().expect("failed to get current path");

    let config_file_path = format!("{}/{}", current_path.display(), CONFIG_FILENAME);

    match &args.get_one::<bool>("init").unwrap() {
        true => {
            let config = Config::new();
            let config_str = json!(config);
            let _ = fs::write(config_file_path, config_str.to_string());
            println!("{} created", CONFIG_FILENAME);
        }
        false => {
            let config_file = match fs::read_to_string(&config_file_path) {
                Ok(value) => value,
                Err(err) => {
                    match err.kind() {
                        ErrorKind::NotFound => println!("{} not found", CONFIG_FILENAME),
                        _ => panic!("{}", err),
                    };
                    String::from("")
                }
            };

            if config_file == "" {
                return;
            }

            let config_json: Config = serde_json::from_str(&config_file).expect("error parse json");

            for item in config_json.dotfiles.into_iter() {
                let normalize_path = match &config_json.dot_dir {
                    Some(dir) => format!("{}/{}/{}", current_path.display(), dir, item.name),
                    None => format!("{}/{}", current_path.display(), item.name),
                };

                if fs::read_dir(&item.out_dir).is_err() {
                    fs::create_dir_all(&item.out_dir).expect("failed to create folder");
                }

                process::Command::new("ln")
                    .arg("-sf")
                    .arg(&normalize_path)
                    .arg(&item.out_dir)
                    .spawn()
                    .expect("error");

                println!("Create Link to {} -> {}", normalize_path, item.out_dir);
            }
        }
    }
}
