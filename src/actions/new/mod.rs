use crate::actions::api;
use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn init() {
    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name:")
        .interact_text()
        .unwrap();

    let selections = &["API", "Blank"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .with_prompt("Project type:")
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection].to_lowercase().as_str() {
        "api" => api::init(project_name),
        _ => println!("Select is not found"),
    }
}
