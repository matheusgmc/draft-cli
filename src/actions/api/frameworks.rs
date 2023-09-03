use std::collections::HashMap;

use crate::utils::dependency::Dependency;

pub struct Frameworks {
    pub data: HashMap<String, Dependency>,
    pub default: Dependency,
}

impl Frameworks {
    pub fn new() -> Frameworks {
        let map = Dependency::select_from(vec![
            Dependency::new("Express")
                .set_type("@types/express")
                .set_template("express/express", "index"),
            Dependency::new("Fastify"),
            Dependency::new("Express with cors")
                .set_type("@types/express")
                .set_type("@types/cors")
                .set_template("express/express-cors", "index")
                .set_command("npm", vec!["install", "express"]),
        ]);

        Frameworks {
            default: map.get(&String::from("express")).unwrap().clone(),
            data: map,
        }
    }

    pub fn get_labels(&self) -> Vec<String> {
        let mut vec: Vec<String> = vec![];
        for i in self.data.values() {
            vec.push(i.name.to_string());
        }
        vec
    }
}
