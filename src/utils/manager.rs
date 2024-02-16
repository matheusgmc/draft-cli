#[derive(Debug, Clone)]
pub struct Manager {
    pub name: String,
    pub install: String,
    pub dev_install: String,
}

impl Manager {
    pub fn new(name: &str, install: &str, dev_install: &str) -> Manager {
        Manager {
            name: name.to_string(),
            install: install.to_string(),
            dev_install: dev_install.to_string(),
        }
    }

    pub fn build() -> Vec<Manager> {
        let mut items = vec![];

        items.push(Manager::default());
        items.push(Manager::new("npm", "install", "install -D"));
        items.push(Manager::new("yarn", "install", "install -D"));

        items
    }

    pub fn default() -> Manager {
        Manager::new("pnpm", "install", "install -D")
    }

    pub fn get_labels(vec: &Vec<Manager>) -> Vec<String> {
        vec.iter().map(|e| e.name.clone()).collect::<Vec<String>>()
    }
}
