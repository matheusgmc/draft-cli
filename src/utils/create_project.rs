use std::{
    env, fs,
    path::{self},
    process,
};

use super::project::Project;

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

    let typescript = match project.typescript {
        true => true,
        _ => {
            process::Command::new("npm")
                .current_dir(project_folder)
                .args(["pkg", "set", "type=module"])
                .output()
                .expect("error in set type module");
            false
        }
    };

    println!("Installing {} dependencies", project.dependencies.len());
    for dependency in project.dependencies.iter() {
        let types = dependency.package.starts_with("@types");

        if dependency.dev {
            if types && !typescript {
                return;
            };

            process::Command::new("npm")
                .current_dir(project_folder)
                .args(["install", "-D"])
                .arg(&dependency.package)
                .output()
                .expect("error in install types");
        } else {
            process::Command::new("npm")
                .current_dir(project_folder)
                .arg("install")
                .arg(&dependency.package)
                .output()
                .expect("error in install types");
        }

        if dependency.template.is_some() {
            dependency
                .template
                .as_ref()
                .unwrap()
                .create_template(project_folder.display().to_string());
        }
    }

    println!();
    println!("Done");
    println!("Your project was created in {}", test);
    println!("cd {}", project.name);
}
