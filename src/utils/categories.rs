use std::collections::HashMap;

use super::dependencies::{Dependencies, Dependency};

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub dependencies: HashMap<String, Vec<Dependency>>,
}

impl Category {
    pub fn new(name: &str, dependencies: HashMap<String, Vec<Dependency>>) -> Self {
        Category {
            name: name.to_string(),
            dependencies,
        }
    }
}

pub struct Categories {
    pub items: HashMap<String, Category>,
}

impl Categories {
    pub fn build(entry_point: &String) -> Self {
        let dependencies = Dependencies::build(entry_point);
        let mut items: HashMap<String, Category> = HashMap::new();

        items.insert(
            String::from("BLANK"),
            Category::new("BLANK", HashMap::new()),
        );
        items.insert(String::from("API"), Category::new("API", dependencies.api));

        Categories { items }
    }

    pub fn default(self) -> Category {
        self.items.get("BLANK").unwrap().clone()
    }

    pub fn get_labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = vec![];

        for key in self.items.keys() {
            labels.push(key.to_string());
        }

        labels
    }
}
