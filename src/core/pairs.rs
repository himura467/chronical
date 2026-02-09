use std::collections::HashMap;
use std::fmt;

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

    pub fn get_parsed<T, E>(&self, key: &str) -> Result<Option<T>, E>
    where
        T: std::str::FromStr<Err = E>,
    {
        self.get(key).map(|v| v.parse()).transpose()
    }

    pub fn get_csv<T, E>(&self, key: &str) -> Result<Option<Vec<T>>, E>
    where
        T: std::str::FromStr<Err = E>,
    {
        self.get(key)
            .map(|v| v.split(',').map(|s| s.parse()).collect())
            .transpose()
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}

impl fmt::Display for Pairs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (key, value) in &self.pairs {
            if !first {
                write!(f, ";")?;
            }
            write!(f, "{}={}", key, value)?;
            first = false;
        }
        Ok(())
    }
}
