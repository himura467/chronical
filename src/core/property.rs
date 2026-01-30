use super::pairs::Pairs;
use crate::error::ParseError;
use std::str::FromStr;

/// Property value
pub enum Value {
    Single(String),
    Pairs(Pairs),
}

/// iCalendar property
///
/// Format: name *(";" param ) ":" value CRLF
///
/// Example:
/// ```text
/// ATTENDEE;RSVP=TRUE;ROLE=REQ-PARTICIPANT:mailto:jsmith@example.com
/// RDATE;VALUE=DATE:19970304,19970504,19970704,19970904
/// ```
pub struct Property {
    /// The property name (e.g., DTSTART, RRULE, RDATE, EXRULE, EXDATE)
    pub name: String,
    /// Property parameters as key-value pairs (e.g., TZID=America/New_York)
    pub parameters: Pairs,
    /// The property value
    pub value: Value,
}

impl Property {
    pub fn new(name: String, parameters: Pairs, value: Value) -> Self {
        let name = name.to_uppercase();

        Self {
            name,
            parameters,
            value,
        }
    }

    fn parse_value(value: &str) -> Result<Value, ParseError> {
        if !value.contains('=') {
            if value.contains(';') {
                return Err(ParseError::InvalidProperty(format!(
                    "Invalid value format: {}",
                    value
                )));
            }
            return Ok(Value::Single(value.to_string()));
        }

        let parts: Vec<&str> = value.split(';').collect();
        let mut pairs = Pairs::new();
        for part in &parts {
            let (k, v) = part.split_once('=').ok_or_else(|| {
                ParseError::InvalidProperty(format!("Invalid value format: {}", value))
            })?;
            pairs.insert(k.to_string(), v.to_string());
        }
        Ok(Value::Pairs(pairs))
    }
}

impl FromStr for Property {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Find the first colon that separates name+params from value
        let colon_pos = s.find(':').ok_or_else(|| {
            ParseError::InvalidProperty(format!("Invalid property format (missing colon): {}", s))
        })?;

        let name_and_params = &s[..colon_pos];
        let value = &s[colon_pos + 1..];

        let mut parts = name_and_params.split(';');
        let name = parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| {
                ParseError::InvalidProperty(format!(
                    "Invalid property format (missing name): {}",
                    s
                ))
            })?
            .to_uppercase();

        let mut parameters = Pairs::new();
        for param in parts {
            let (k, v) = param
                .split_once('=')
                .filter(|(k, v)| !k.is_empty() && !v.is_empty())
                .ok_or_else(|| {
                    ParseError::InvalidProperty(format!("Invalid parameter format: {}", param))
                })?;

            parameters.insert(k.to_string(), v.to_string());
        }

        let value = Self::parse_value(value)?;

        Ok(Property {
            name,
            parameters,
            value,
        })
    }
}
