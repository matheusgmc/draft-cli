use clap::Command;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::utils::{dependency::Dependency, package_manager, project::Project, suport::Suport};

use super::{api, blank};

pub fn command() -> Command {
    Command::new("new").about("create new projects")
}

pub fn handle() {
    let mut project = Project {
        name: String::from("my_project"),
        entry_point: String::from("index"),
        typescript: true,
        category: String::from("Blank"),
        dependencies: vec![],
        manager: package_manager::default(),
    };

    project.name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Project name:")
        .default(project.name)
        .interact_text()
        .unwrap();

    let selections = &["Blank", "API"];

    let select = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project type:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    project.category = selections[select].to_string();

    project.typescript = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Wil you use TypeScript?")
        .default(true)
        .interact_opt()
        .unwrap()
        .unwrap();

    if project.typescript {
        let suport = Suport::new(project.typescript, &mut project);

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

        project.dependencies.push(item.to_owned());
    }

    let managers = package_manager::get_managers();

    let manager_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("run the project using:")
        .default(0)
        .items(&package_manager::get_labels())
        .interact()
        .unwrap();

    project.manager = managers.get(manager_index).unwrap().to_owned();

    match project.category.to_lowercase().as_str() {
        "api" => api::handle(&mut project),
        "blank" => blank::handle(&mut project),
        _ => println!("Select is not found"),
    }
}
