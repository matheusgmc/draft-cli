use std::collections::HashMap;

use crate::templates::Template;

#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: String,
    pub package: String,
    pub dev: bool,
    pub template: Option<Template>,
    pub scripts: Vec<String>,
}

impl Dependency {
    pub fn new(name: &str) -> Dependency {
        Dependency {
            name: name.to_string(),
            package: name.to_lowercase(),
            dev: false,
            template: None,
            scripts: vec![],
        }
    }

    pub fn new_with_packages(name: &str, package: &str) -> Dependency {
        Dependency {
            name: name.to_string(),
            package: package.to_string(),
            dev: false,
            template: None,
            scripts: vec![],
        }
    }

    pub fn add_script(mut self, v: String) -> Self {
        self.scripts.push(v);
        self
    }

    pub fn set_template(mut self, file_path: &str, file_name: &str) -> Self {
        self.template = Some(Template {
            file_path: String::from(file_path),
            file_name: String::from(file_name),
        });
        self
    }

    pub fn dev(mut self) -> Self {
        self.dev = true;

        self
    }
}

pub struct Dependencies {
    pub api: HashMap<String, Vec<Dependency>>,
    pub extras: HashMap<String, Vec<Dependency>>,
}

impl Dependencies {
    pub fn build(entry_point: &String) -> Dependencies {
        let mut api = HashMap::new();
        let mut extras = HashMap::new();

        api.insert(
            String::from("Express"),
            vec![
                Dependency::new("express"),
                Dependency::new("@types/express")
                    .set_template("express", entry_point)
                    .dev(),
            ],
        );

        api.insert(
            String::from("Express with Cors"),
            vec![
                Dependency::new("express"),
                Dependency::new("cors"),
                Dependency::new("@types/cors").dev(),
                Dependency::new("@types/express")
                    .set_template("express-cors", entry_point)
                    .dev(),
            ],
        );

        api.insert(
            String::from("Fastify"),
            vec![Dependency::new("fastify").set_template("fastify", entry_point)],
        );

        extras.insert(
            String::from("Prisma - ORM"),
            vec![
                Dependency::new("@prisma/client"),
                Dependency::new("prisma").dev(),
            ],
        );

        extras.insert(
            String::from("Jest - Testing"),
            vec![Dependency::new("jest").dev()],
        );

        Dependencies { api, extras }
    }

    pub fn get_labels(hash: &HashMap<String, Vec<Dependency>>) -> Vec<String> {
        hash.keys().map(|e| e.to_string()).collect()
    }

    pub fn running_ts(entry_point: &String, typescript: &bool) -> HashMap<String, Vec<Dependency>> {
        let mut running_ts = HashMap::new();

        let ext = match typescript {
            true => "ts",
            false => "js",
        };

        running_ts.insert(
            String::from("tsx"),
            vec![Dependency::new("tsx")
                .add_script(format!(
                    "scripts.dev=npx tsx watch src/{}.{}",
                    entry_point, ext
                ))
                .dev()],
        );
        running_ts.insert(
            String::from("ts-node-dev"),
            vec![Dependency::new("ts-node-dev")
                .add_script(format!(
                    "scripts.dev=npx tsnd --respawn --transpile-only src/{}.{}",
                    entry_point, ext
                ))
                .dev()],
        );

        running_ts
    }
}
