use super::{
    categories::{Categories, Category},
    dependencies::Dependency,
    manager::Manager,
};

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub entry_point: String,
    pub category: Category,
    pub dependencies: Vec<Dependency>,
    pub typescript: bool,
    pub manager: Manager,
}

impl Project {
    pub fn default(name: &Option<&String>, ts: &Option<&bool>, category: &Option<&String>) -> Self {
        Project {
            name: name.unwrap_or(&String::from("my_project")).to_string(),
            entry_point: String::from("index"),
            category: Categories::build(&String::from("index"))
                .items
                .get(&category.unwrap_or(&String::from("BLANK")).to_uppercase())
                .unwrap()
                .clone(),
            typescript: *ts.unwrap_or(&true),
            dependencies: vec![],
            manager: Manager::default(),
        }
    }
}
