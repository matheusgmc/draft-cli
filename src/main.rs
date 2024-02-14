use clap::Command;
pub mod commands;
pub mod templates;
pub mod utils;

fn main() {
    let matches = Command::new("draft-cli")
        .about("Create projects")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("matheusgmc")
        .subcommand(commands::new::command_new())
        .get_matches();

    match matches.subcommand() {
        Some(("new", _)) => commands::new::handle(),
        _ => unreachable!(),
    }
}
