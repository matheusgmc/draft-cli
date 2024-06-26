use clap::Command;
pub mod commands;
pub mod templates;
pub mod utils;

fn main() {
    let matches = Command::new("draft-cli")
        .about("Create projects")
        .version("0.2.5")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("matheusgmc")
        .subcommand(commands::new::command_new())
        .subcommand(commands::gm::command_gm())
        .subcommand(commands::dotfiles::command_dotfiles())
        .get_matches();

    match matches.subcommand() {
        Some(("new", args)) => commands::new::handle(args),
        Some(("gc", _)) => commands::gm::handle(),
        Some(("dotfiles", args)) => commands::dotfiles::handle(args),
        _ => unreachable!(),
    }
}
