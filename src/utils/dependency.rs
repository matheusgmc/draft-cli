use crate::{templates::Template, utils::command::Command};

use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub commands: Option<Vec<Command>>,
    pub types: Option<Vec<String>>,
    pub template: Option<Template>,
}

impl Dependency {
    pub fn new(name: &str) -> Dependency {
        Dependency {
            name: String::from(name),
            commands: None,
            types: None,
            template: None,
        }
    }

    pub fn set_type(mut self, types: &str) -> Self {
        if self.types.is_none() {
            self.types = Some(vec![String::from(types)]);
        } else {
            self.types.as_mut().unwrap().push(String::from(types));
        }
        self
    }

    pub fn set_template(mut self, file_path: &str, file_name: &str) -> Self {
        self.template = Some(Template {
            file_path: String::from(file_path),
            file_name: String::from(file_name),
        });
        self
    }

    pub fn set_command<I, S>(mut self, name: &str, args_str: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args = args_str
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        if self.commands.is_none() {
            self.commands = Some(vec![Command {
                name: String::from(name),
                args,
            }]);
        } else {
            self.commands.as_mut().unwrap().push(Command {
                name: String::from(name),
                args,
            });
        }
        self
    }

    pub fn select_from(vec: Vec<Dependency>) -> HashMap<String, Dependency> {
        let mut map: HashMap<String, Dependency> = HashMap::new();
        for i in vec {
            map.insert(i.name.to_lowercase(), i);
        }
        map
    }

    pub fn get_labels(map: &HashMap<String, Dependency>) -> Vec<String> {
        let mut vec: Vec<String> = vec![];
        for i in map.values() {
            vec.push(i.name.to_string());
        }
        vec
    }
}
