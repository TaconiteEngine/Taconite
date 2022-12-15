use std::{collections::HashMap, path::Path};

#[derive(Default)]
pub struct ResourceHandler {
    resources: HashMap<String, String>,
}

impl ResourceHandler {
    pub fn add_resource(&mut self, name: &str, path: &Path) {
        if path.exists() {
            self.resources
                .insert(name.to_string(), path.to_str().unwrap().to_string());
        } else {
            println!("Path for resource {name} does not exist.");
        }
    }

    pub fn get_resource_path(&self, resource_name: &str) -> &Path {
        Path::new(self.resources.get(resource_name).unwrap())
    }
}
