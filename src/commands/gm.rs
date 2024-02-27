use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    env,
    process::{self, Stdio},
};

use clap::Command;
use regex::Regex;

pub fn command_gm() -> Command {
    Command::new("gc").about("Generate a commit message by git stagged")
}

pub fn handle() {
    let current_path = env::current_dir().unwrap().display().to_string();

    if process::Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .expect("Failed to get git diff")
        .stdout
        .is_empty()
    {
        println!("Your staging is empty");
        return;
    }

    let stagged_process = process::Command::new("git")
        .current_dir(&current_path)
        .args(["diff", "--cached", "-U0", "--no-prefix"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("error git diff");

    let gpt_process = process::Command::new("tgpt")
            .args([
                "-q",
                "-w",
                "suggest 10 commit messages based on the following diff\n - follow conventional commits:\n - no scope",
            ])
            .stdin(Stdio::from(stagged_process.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to generate commit with gpt");

    let gpt_output = gpt_process
        .wait_with_output()
        .expect("Failed wait for gpt output");

    let content = String::from_utf8(gpt_output.stdout).unwrap();

    let regex_to_suggestions = Regex::new(r"^\d+\..+").unwrap();

    let suggestions = content
        .lines()
        .filter(|e| regex_to_suggestions.is_match(e))
        .collect::<Vec<&str>>();

    let index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick a message:")
        .default(0)
        .items(&suggestions)
        .interact()
        .unwrap();

    let regex_to_message = Regex::new(r"`([^`]+)`").unwrap();

    let captures = regex_to_message.captures(suggestions[index]);

    let message = captures.unwrap().get(1).unwrap();

    process::Command::new("git")
        .current_dir(current_path)
        .args(["commit", "-m", message.as_str()])
        .spawn()
        .expect("Failed to commit message");
}
