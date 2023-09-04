use crate::actions::new::Project;

use std::{env, fs, path, process};

pub fn main(project: &mut Project) {
    let current_path = env::current_dir().unwrap();
    let test = format!("{}/{}", current_path.display(), &project.name);
    let project_folder = path::Path::new(&test);

    println!("Creating project folder in {}", test);
    fs::create_dir(project_folder).unwrap();

    process::Command::new("node")
        .arg("-v")
        .output()
        .expect("node is not instaled");

    // env::set_current_dir(project_folder).unwrap();

    println!("Initializing the project using npm");
    process::Command::new("npm")
        .current_dir(project_folder)
        .arg("init")
        .arg("-y")
        .output()
        .expect("npm is not instaled");

    match project.dependencies.last().unwrap().name.as_str() {
        "typescript" => {
            process::Command::new("npm")
                .current_dir(project_folder)
                .args(["pkg", "set", "type=module"])
                .output()
                .expect("error in set type module");
        }
        _ => {}
    };
    println!("Installing {} dependencies", project.dependencies.len());
    for dependency in project.dependencies.iter() {
        if dependency.commands.is_some() {
            for command in dependency.commands.as_ref().unwrap().iter() {
                process::Command::new(&command.name)
                    .current_dir(project_folder)
                    .args(&command.args)
                    .output()
                    .expect("this dependency is not found");
            }
        } else {
            process::Command::new("npm")
                .current_dir(project_folder)
                .arg("install")
                .arg(dependency.name.to_lowercase())
                .output()
                .expect("this dependency is not found");
        }

        if dependency.types.is_some() {
            process::Command::new("npm")
                .current_dir(project_folder)
                .args(["install", "-D"])
                .args(dependency.types.as_ref().unwrap())
                .output()
                .expect("error in install types");
        }

        if dependency.template.is_some() {
            dependency
                .template
                .as_ref()
                .unwrap()
                .create_template(project_folder.display().to_string())
        }
    }

    println!();
    println!("Done");
    println!("Your project was created in {}", test);
    println!("cd {}", project.name);
}
