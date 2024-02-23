use chrono::prelude::*;
use std::collections::HashMap;

pub struct Cache {
    pub index: HashMap<String, (NaiveDate, String)>,
}

impl Cache {
    pub fn add(&mut self, key: String, value: (NaiveDate, String)) {
        self.index.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        if let Some(value) = self.index.get(&key) {
            let (_, file) = value;
            Some(file.to_string())
        } else {
            None
        }
    }
}
