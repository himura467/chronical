use std::collections::HashMap;

/// Property parameters
///
/// Property parameters contain meta-information about the property or the property value.
pub struct Parameters {
    parameters: HashMap<String, String>,
}

impl Parameters {
    pub fn new() -> Self {
        Self {
            parameters: HashMap::new(),
        }
    }

    /// Insert a parameter with the given key and value
    ///
    /// Per RFC 5545:
    /// - Keys are case-insensitive (stored in uppercase)
    /// - Quoted values (e.g., "John Smith") are case-sensitive (quotes removed, case preserved)
    /// - Unquoted values are case-insensitive (stored in uppercase)
    pub fn insert(&mut self, key: String, value: String) {
        let normalized_key = key.to_uppercase();

        let normalized_value = if value.starts_with('"') && value.ends_with('"') && value.len() >= 2
        {
            value[1..value.len() - 1].to_string()
        } else {
            value.to_uppercase()
        };

        self.parameters.insert(normalized_key, normalized_value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.parameters.get(&key.to_uppercase())
    }
}
