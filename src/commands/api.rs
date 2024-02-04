use crate::utils::{create_project, project::Project};

pub fn handle(project: &mut Project) {
    create_project::build(project)
}
