use super::{
    categories::{Categories, Category},
    dependencies::Dependency,
};

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub entry_point: String,
    pub category: Category,
    pub dependencies: Vec<Dependency>,
    pub typescript: bool,
}

impl Project {
    pub fn default() -> Self {
        Project {
            name: String::from("my_project"),
            entry_point: String::from("index"),
            category: Categories::build().default(),
            typescript: true,
            dependencies: vec![],
        }
    }
}
