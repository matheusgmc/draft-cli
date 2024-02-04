use super::{dependency::Dependency, package_manager::PackageManager};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub entry_point: String,
    pub typescript: bool,
    pub category: String,
    pub dependencies: Vec<Dependency>,
    pub manager: PackageManager,
}
