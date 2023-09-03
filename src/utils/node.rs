use std::collections::HashMap;

use super::dependency::Dependency;

pub struct Node {
    pub available_dependencies: HashMap<String, Dependency>,
    pub dependencies: Vec<Dependency>,
}

impl Node {
    pub fn default() -> Dependency {
        Dependency::new("Node")
            .set_command("npm", vec!["pkg", "set", "scripts.start=node src/index.js"])
    }

    pub fn new() -> Node {
        Node {
            dependencies: vec![],
            available_dependencies: Dependency::select_from(vec![Dependency::new("nodemon")
                .set_command(
                    "npm",
                    vec!["pkg", "set", "scripts.dev=npx nodemon src/index.js"],
                )]),
        }
    }

    pub fn get_labels_available_dependencies(&self) -> Vec<String> {
        let mut vec = vec![];

        for i in self.available_dependencies.values() {
            vec.push(i.name.clone());
        }

        vec
    }
}
