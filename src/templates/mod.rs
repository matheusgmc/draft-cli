use std::fs;

#[derive(Debug, Clone)]
pub struct Template {
    pub file_path: String,
    pub file_name: String,
}

impl Template {
    pub fn new(file: &str, file_name: &str) -> Template {
        Template {
            file_path: String::from(file), // express/express or  express/express-cors
            file_name: String::from(file_name),
        }
    }

    pub fn create_template(&self, project_path: String, typescript: bool) {
        let src_path = format!("{}/src", project_path);
        if fs::read_dir(&src_path).is_err() {
            fs::create_dir(&src_path).expect("error in create src folder");
        }

        let path_templates = format!("./src/templates/{}.tpl", self.file_path);
        let mut file_name = self.file_name.clone();

        if typescript {
            file_name.push_str(".ts");
        } else {
            file_name.push_str(".js");
        }
        let content = fs::read_to_string(&path_templates).expect("Error in read template");

        let path_file_name = format!("{}/{}", src_path, file_name);

        fs::write(path_file_name, content).expect("error in create index file")
    }
}
