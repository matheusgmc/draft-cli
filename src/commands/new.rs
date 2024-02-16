use clap::Command;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

use crate::utils::{
    categories::Categories,
    create_project,
    dependencies::{Dependencies, Dependency},
    manager::Manager,
    project::Project,
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
        .with_prompt("will you use typescript?")
        .default(project.typescript)
        .interact_opt()
        .unwrap()
        .is_some();

    let managers = Manager::build();
    let manager_labels = Manager::get_labels(&managers);

    let manager_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("package manager:")
        .default(0)
        .items(&manager_labels)
        .interact()
        .unwrap();

    project.manager = managers[manager_index].clone();

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
            .extend(running.get(&running_labels[running_index]).unwrap().clone());

        project.dependencies.extend(vec![
            Dependency::new("typescript").dev(),
            Dependency::new("@types/node").dev(),
        ]);
    }

    let categories = Categories::build(&project.entry_point);
    let categories_labels = &categories.get_labels();

    let categories_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("project type: ")
        .default(0)
        .items(&categories_labels)
        .interact()
        .unwrap();

    project.category = categories
        .items
        .get(&categories_labels[categories_index])
        .unwrap()
        .clone();

    if !project.category.dependencies.is_empty() {
        let dependencies_labels = Dependencies::get_labels(&project.category.dependencies);
        let dependencies_index = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("select a framework:")
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
    }

    let extras = Dependencies::build(&project.entry_point).extras;
    let extras_labels = Dependencies::get_labels(&extras);

    let extras_index = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("extras (press 'space' to select):")
        .defaults(&[])
        .items(&extras_labels)
        .interact()
        .unwrap();

    for index in extras_index {
        project
            .dependencies
            .extend(extras.get(&extras_labels[index]).unwrap().clone())
    }

    create_project::main(&mut project)
}
