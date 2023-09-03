use std::collections::HashMap;

use crate::utils::dependency::Dependency;

#[derive(Debug)]
pub struct Typescript {
    pub available_dependencies: HashMap<String, Dependency>,
    pub dependencies: Vec<Dependency>,
}

impl Typescript {
    pub fn default() -> Dependency {
        Dependency::new("Typescript")
    }
    pub fn new() -> Typescript {
        Typescript {
            dependencies: vec![],
            available_dependencies: Dependency::select_from(vec![
                Dependency::new("tsx").set_command(
                    "npm",
                    vec!["pkg", "set", "scripts.dev=npx tsx watch src/index.ts"],
                ),
                Dependency::new("ts-node-dev")
                    .set_type("@types/node")
                    .set_command(
                        "npm",
                        vec![
                            "pkg",
                            "set",
                            "scripts.dev=npx tsnd --respawn --transpile-only src/index.ts",
                        ],
                    ),
            ]),
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
