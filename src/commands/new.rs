use clap::Command;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::utils::{
    categories::Categories, create_project, dependencies::Dependencies, project::Project,
};

pub fn command_new() -> Command {
    Command::new("new").about("Create a new project")
}

pub fn handle() {
    let mut project = Project::default();

    project.name = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("project name: ")
        .default(project.name)
        .interact_text()
        .unwrap();

    project.entry_point = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("entry point: ")
        .default(project.entry_point)
        .interact_text()
        .unwrap();

    project.typescript = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Wil you use TypeScript?")
        .default(project.typescript)
        .interact_opt()
        .unwrap()
        .is_some();

    if project.typescript {
        let running = Dependencies::running_ts();
        let running_labels = Dependencies::get_labels(&running);
        let running_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("run the project using:")
            .default(0)
            .items(&running_labels)
            .interact()
            .unwrap();
        project
            .dependencies
            .extend(running.get(&running_labels[running_index]).unwrap().clone())
    }

    let categories = Categories::build();
    let categories_labels = &categories.get_labels();

    let categories_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Project Type: ")
        .default(0)
        .items(&categories_labels)
        .interact()
        .unwrap();

    project.category = categories
        .items
        .get(&categories_labels[categories_index])
        .unwrap()
        .clone();

    let dependencies_labels = Dependencies::get_labels(&project.category.dependencies);
    let dependencies_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a framework:")
        .default(0)
        .items(&dependencies_labels)
        .interact()
        .unwrap();

    project.dependencies.extend(
        project
            .category
            .dependencies
            .get(&dependencies_labels[dependencies_index])
            .unwrap()
            .clone(),
    );

    create_project::main(&mut project)
}
