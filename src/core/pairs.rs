use std::collections::HashMap;

/// Key-value pairs
pub struct Pairs {
    pairs: HashMap<String, String>,
}

impl Pairs {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
        }
    }

    /// Insert a pair with the given key and value
    pub fn insert(&mut self, key: String, value: String) {
        let normalized_key = key.to_uppercase();

        let normalized_value = if value.starts_with('"') && value.ends_with('"') && value.len() >= 2
        {
            value[1..value.len() - 1].to_string()
        } else {
            value
        };

        self.pairs.insert(normalized_key, normalized_value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.pairs.get(&key.to_uppercase())
    }
}
