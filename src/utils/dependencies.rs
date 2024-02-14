use std::collections::HashMap;

use crate::templates::Template;

#[derive(Clone, Debug)]
pub struct Dependency {
    pub name: String,
    pub package: String,
    pub dev: bool,
    pub template: Option<Template>,
}

impl Dependency {
    pub fn new(name: &str) -> Dependency {
        Dependency {
            name: name.to_string(),
            package: name.to_lowercase(),
            dev: false,
            template: None,
        }
    }

    pub fn new_with_packages(name: &str, package: &str) -> Dependency {
        Dependency {
            name: name.to_string(),
            package: package.to_string(),
            dev: false,
            template: None,
        }
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
}

impl Dependencies {
    pub fn build(entry_point: &String) -> Dependencies {
        let mut api = HashMap::new();

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

        Dependencies { api }
    }

    pub fn get_labels(hash: &HashMap<String, Vec<Dependency>>) -> Vec<String> {
        hash.keys().map(|e| e.to_string()).collect()
    }

    pub fn running_ts() -> HashMap<String, Vec<Dependency>> {
        let mut running_ts = HashMap::new();

        running_ts.insert(String::from("tsx"), vec![Dependency::new("tsx").dev()]);
        running_ts.insert(
            String::from("ts-node-dev"),
            vec![Dependency::new("tsx").dev()],
        );

        running_ts
    }
}
