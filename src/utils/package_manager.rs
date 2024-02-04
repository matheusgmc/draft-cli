#[derive(Debug, Clone)]
pub struct PackageManager {
    pub name: String,
    pub install: String,
    pub install_dev: String,
    pub exec: String,
}

impl PackageManager {
    pub fn new(name: &str, install: &str, dev: &str, exec: &str) -> PackageManager {
        PackageManager {
            name: String::from(name),
            install: String::from(install),
            install_dev: String::from(dev),
            exec: String::from(exec),
        }
    }
}

pub fn default() -> PackageManager {
    PackageManager::new("NPM", "npm install", "npm install -D", "npx")
}

pub fn get_managers() -> Vec<PackageManager> {
    let managers = vec![default()];
    managers
}

pub fn get_labels() -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    for i in get_managers() {
        vec.push(i.name.to_string());
    }
    vec
}
