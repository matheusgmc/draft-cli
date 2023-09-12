use crate::utils::create_project;

use super::new::Project;

pub fn init(project: &mut Project) {
    let first = project.dependencies.first();
    match first {
        Some(ts) => {
            let new = ts.to_owned().set_template("blank/blank", "index");
            project.dependencies[0] = new;
        }
        None => {}
    }

    println!("{:#?}", project);
    create_project::main(project);
}
