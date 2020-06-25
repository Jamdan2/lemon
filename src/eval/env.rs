use std::collections::HashMap;
use crate::eval::Object;

pub struct Env {
    map: HashMap<String, Object>
}

impl Env {
    pub fn new() -> Self {
        Env {
            map: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        // Check later
        self.map.get(key).map(|value| *value.clone())
    }
}
