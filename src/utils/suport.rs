use std::collections::HashMap;

use crate::actions::api::Args;

use super::dependency::Dependency;

pub struct Suport {
    pub dependencies: HashMap<String, Dependency>,
}

impl Suport {
    fn typescript_dependencies() -> Vec<Dependency> {
        vec![
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
        ]
    }

    fn node_dependencies() -> Vec<Dependency> {
        vec![Dependency::new("nodemon").set_command(
            "npm",
            vec!["pkg", "set", "scripts.dev=npx nodemon src/index.js"],
        )]
    }

    pub fn new(args: &mut Args) -> Suport {
        let dependencies = match args.typescript {
            true => {
                args.dev_dependencies.push(Dependency::new("typescript"));
                Suport::typescript_dependencies()
            }
            false => Suport::node_dependencies(),
        };

        Suport {
            dependencies: Dependency::select_from(dependencies),
        }
    }
}
