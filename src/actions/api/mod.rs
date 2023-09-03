use dialoguer::{theme::ColorfulTheme, Confirm, Select};

use crate::actions::api::frameworks::Frameworks;
use crate::utils::dependency::Dependency;
use crate::utils::suport::Suport;

pub mod create_project;
pub mod frameworks;

#[derive(Debug, Clone)]
pub struct Args {
    pub name: String,
    pub dev_dependencies: Vec<Dependency>,
    pub dependencies: Vec<Dependency>,
    pub typescript: bool,
}

pub fn init(name: String) {
    let frameworks = Frameworks::new();
    let labels = frameworks.get_labels();

    let mut args = Args {
        name,
        dev_dependencies: vec![],
        dependencies: vec![],
        typescript: true,
    };

    args.typescript = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Wil you use TypeScript?")
        .default(args.typescript)
        .interact_opt()
        .unwrap()
        .unwrap();

    let suport = Suport::new(&mut args);

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

    args.dev_dependencies.push(item.clone());

    let framework = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a framework:")
        .default(0)
        .items(&labels[..])
        .interact()
        .unwrap();

    args.dependencies.push(
        frameworks
            .data
            .get(&labels[framework].to_lowercase())
            .unwrap()
            .clone(),
    );

    create_project::main(&args);
}
