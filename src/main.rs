use clap::Command;

pub mod actions;
pub mod templates;
pub mod utils;

fn command_new() -> Command {
    Command::new("new").about("create new projects")
}

fn main() {
    let matches = Command::new("CLI-Draft")
        .about("manager project Node.js")
        .version("1.0.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("matheusgmc")
        .subcommand(command_new())
        .get_matches();

    match matches.subcommand() {
        Some(("new", _)) => {
            actions::new::init();
        }
        _ => unreachable!(),
    }
}
