use crate::{actions::api, utils::dependency::Dependency};
use dialoguer::{theme::ColorfulTheme, Input, Select};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub category: String,
    pub dependencies: Vec<Dependency>,
}

pub fn init() {
    let mut project = Project {
        name: String::from("my_project"),
        category: String::from("Blank"),
        dependencies: vec![],
    };

    project.name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name:")
        .default(project.name)
        .interact_text()
        .unwrap();

    let selections = &["API", "Blank"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .with_prompt("Project type:")
        .items(&selections[..])
        .interact()
        .unwrap();

    project.category = selections[select].to_string();

    match project.category.to_lowercase().as_str() {
        "api" => api::init(&mut project),
        _ => println!("Select is not found"),
    }
}
