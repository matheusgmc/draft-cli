use dialoguer::{theme::ColorfulTheme, Confirm, Select};

use crate::actions::api::frameworks::Frameworks;
use crate::utils::dependency::Dependency;
use crate::utils::suport::Suport;

use super::new::Project;

pub mod create_project;
pub mod frameworks;

pub fn init(project: &mut Project) {
    let frameworks = Frameworks::new();
    let labels = frameworks.get_labels();

    let typescript = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Wil you use TypeScript?")
        .default(false)
        .interact_opt()
        .unwrap()
        .unwrap();

    let suport = Suport::new(typescript, &mut project.dependencies);

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

    let framework = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a framework:")
        .default(0)
        .items(&labels[..])
        .interact()
        .unwrap();

    project.dependencies.push(
        frameworks
            .data
            .get(&labels[framework].to_lowercase())
            .unwrap()
            .clone(),
    );

    create_project::main(project);
}
