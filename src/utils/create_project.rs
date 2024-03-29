use std::{
    env::{self},
    fs,
    path::{self},
    process,
};

use crate::templates::Template;

use super::project::Project;

pub fn main(project: &mut Project) {
    let current_path = env::current_dir().unwrap();
    let test = format!("{}/{}", current_path.display(), &project.name);
    let project_folder = path::Path::new(&test);

    process::Command::new(&project.manager.name)
        .output()
        .expect(&format!("{} is not installed", &project.manager.name));

    println!("Creating project folder in {}", test);
    fs::create_dir(project_folder).unwrap();

    process::Command::new("node")
        .arg("-v")
        .output()
        .expect("node is not installed");

    println!("Initializing the project using npm init");
    process::Command::new("npm")
        .current_dir(project_folder)
        .arg("init")
        .arg("-y")
        .output()
        .expect("npm is not installed");

    let typescript = match project.typescript {
        true => true,
        _ => {
            process::Command::new("npm")
                .current_dir(project_folder)
                .args(["pkg", "set", "type=module"])
                .output()
                .expect("error set type to module");
            false
        }
    };

    if project.category.name == "BLANK" {
        Template::new("blank", &project.entry_point)
            .create_template(project_folder.display().to_string(), project.typescript);
    }

    println!("Installing {} dependencies", project.dependencies.len());
    for dependency in project.dependencies.iter() {
        let types = dependency.package.starts_with("@types");

        if dependency.dev {
            if types && !typescript {
                return;
            };

            process::Command::new(&project.manager.name)
                .current_dir(project_folder)
                .args(project.manager.dev_install.as_mut().split(" "))
                .arg(&dependency.package)
                .output()
                .expect("error install dev dependencies");
        } else {
            process::Command::new(&project.manager.name)
                .current_dir(project_folder)
                .args(project.manager.install.as_mut().split(" "))
                .arg(&dependency.package)
                .output()
                .expect("error install dependencies");
        }

        if dependency.template.is_some() {
            dependency
                .template
                .as_ref()
                .unwrap()
                .create_template(project_folder.display().to_string(), project.typescript);
        }

        for script in dependency.scripts.iter() {
            process::Command::new("npm")
                .current_dir(project_folder)
                .args(["pkg", "set"])
                .arg(script)
                .output()
                .expect("error set script");
        }
    }

    process::Command::new("git")
        .arg("init")
        .current_dir(project_folder)
        .output()
        .expect("Failed to git init");

    Template::new("gitignore", ".gitignore")
        .create_template(project_folder.display().to_string(), project.typescript);

    println!();
    println!("Done");
    println!("Your project was created in {}", test);
    println!("cd {}", project.name);
}
