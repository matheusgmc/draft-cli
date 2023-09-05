use std::collections::HashMap;

use crate::actions::new::Project;

use super::dependency::Dependency;

pub struct Suport {
    pub dependencies: HashMap<String, Dependency>,
}

impl Suport {
    fn typescript_dependencies(entry_point: &String) -> Vec<Dependency> {
        vec![
            Dependency::new("tsx").set_command(
                "npm",
                [
                    "pkg",
                    "set",
                    format!("scripts.dev=npx tsx watch src/{}.ts", &entry_point).as_str(),
                ],
            ),
            Dependency::new("ts-node-dev")
                .set_type("@types/node")
                .set_command(
                    "npm",
                    [
                        "pkg",
                        "set",
                        format!(
                            "scripts.dev=npx tsnd --respawn --transpile-only src/{}.ts",
                            &entry_point
                        )
                        .as_str(),
                    ],
                ),
        ]
    }

    fn node_dependencies(entry_point: &String) -> Vec<Dependency> {
        vec![Dependency::new("nodemon").set_command(
            "npm",
            [
                "pkg",
                "set",
                format!("scripts.dev=npx nodemon src/{}.js", &entry_point).as_str(),
            ],
        )]
    }

    pub fn new(typescript: bool, project: &mut Project) -> Suport {
        let dependencies = match typescript {
            true => {
                project.dependencies.push(
                    Dependency::new("typescript")
                        .set_command("npm", ["install", "-D", "typescript"])
                        .set_command("npx", ["tsc", "--init"]),
                );
                Suport::typescript_dependencies(&project.entry_point)
            }
            false => Suport::node_dependencies(&project.entry_point),
        };

        Suport {
            dependencies: Dependency::select_from(dependencies),
        }
    }
}
