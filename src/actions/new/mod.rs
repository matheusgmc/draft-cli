use crate::{
    actions::api,
    utils::{dependency::Dependency, suport::Suport},
};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use super::blank;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub entry_point: String,
    pub category: String,
    pub dependencies: Vec<Dependency>,
}

pub fn init() {
    let mut project = Project {
        name: String::from("my_project"),
        category: String::from("Blank"),
        entry_point: String::from("index"),
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

    let typescript = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Wil you use TypeScript?")
        .default(false)
        .interact_opt()
        .unwrap()
        .unwrap();

    let suport = Suport::new(typescript, &mut project);

    let suport_labels = Dependency::get_labels(&suport.dependencies);

    let dependency = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("run the project using:")
        .default(0)
        .items(&suport_labels)
        .interact()
        .unwrap();

    let item = suport
        .dependencies
        .get(&suport_labels[dependency].to_lowercase())
        .unwrap();

    project.dependencies.push(item.clone());

    match project.category.to_lowercase().as_str() {
        "api" => api::init(&mut project),
        "blank" => blank::init(&mut project),
        _ => println!("Select is not found"),
    }
}
