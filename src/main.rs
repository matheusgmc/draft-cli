use clap::Parser;

pub mod actions;
pub mod templates;
pub mod utils;

#[derive(Parser)]
struct Cli {
    action: String,
}

fn main() {
    let args = Cli::parse();

    match args.action.to_lowercase().as_str() {
        "new" => actions::new::init(),
        _ => println!("This action is not exists"),
    };
}
