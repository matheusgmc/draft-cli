use dialoguer::{theme::ColorfulTheme, Select};

use super::new::Project;
use crate::{actions::api::frameworks::Frameworks, utils::create_project};

pub mod frameworks;

pub fn init(project: &mut Project) {
    project.entry_point = String::from("server");

    let frameworks = Frameworks::new();
    let labels = frameworks.get_labels();

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
