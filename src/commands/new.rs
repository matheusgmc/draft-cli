use clap::{Arg, ArgAction, ArgMatches, Command};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};

use crate::utils::{
    categories::Categories,
    create_project,
    dependencies::{Dependencies, Dependency},
    manager::Manager,
    project::Project,
};

pub fn command_new() -> Command {
    Command::new("new")
        .about("Create a new project")
        .arg(
            Arg::new("no-typescript")
                .long("no-ts")
                .action(ArgAction::SetFalse)
                .default_value(None)
                .help("TypeScript is not being used"),
        )
        .arg(
            Arg::new("project")
                .short('p')
                .action(ArgAction::Set)
                .help("Project name"),
        )
        .arg(
            Arg::new("category")
                .short('c')
                .action(ArgAction::Set)
                .value_parser(["api", "blank"])
                .help("Choose the type for your project"),
        )
}
pub fn handle(args: &ArgMatches) {
    let mut project = Project::default(
        &args.get_one::<String>("project"),
        &args.get_one::<bool>("no-typescript"),
        &args.get_one::<String>("category"),
    );

    if args.get_one::<String>("project").is_none() {
        project.name = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("project name: ")
            .default(project.name)
            .interact_text()
            .unwrap();
    }

    project.entry_point = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("entry point: ")
        .default(project.entry_point)
        .interact_text()
        .unwrap();

    if args.get_one::<bool>("no-typescript").is_none() {
        project.typescript = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("will you use typescript?")
            .default(project.typescript)
            .interact_opt()
            .unwrap()
            .is_some();
    }

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
        let running = Dependencies::running_ts(&project.entry_point, &project.typescript);
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
            Dependency::new("typescript")
                .set_template("tsconfig", "tsconfig.json")
                .dev(),
            Dependency::new("@types/node").dev(),
        ]);
    }

    let categories = Categories::build(&project.entry_point);
    if args.get_one::<String>("category").is_none() {
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
    } else {
        project.category = categories
            .items
            .get(&args.get_one::<String>("category").unwrap().to_uppercase())
            .unwrap()
            .clone();
    }

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
