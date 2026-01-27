use super::parameters::Parameters;
use crate::error::ParseError;
use std::str::FromStr;

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
    pub parameters: Parameters,
    /// The property value
    pub value: String,
}

impl Property {
    pub fn new(name: String, parameters: Parameters, value: String) -> Self {
        let name = name.to_uppercase();

        Self {
            name,
            parameters,
            value,
        }
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
        let value = s[colon_pos + 1..].to_string();

        let mut parts = name_and_params.split(';');
        let name = parts
            .next()
            .ok_or_else(|| {
                ParseError::InvalidProperty(format!(
                    "Invalid property format (missing name): {}",
                    s
                ))
            })?
            .to_uppercase();

        let mut parameters = Parameters::new();
        for param in parts {
            let (k, v) = param
                .split_once('=')
                .filter(|(k, v)| !k.is_empty() && !v.is_empty())
                .ok_or_else(|| {
                    ParseError::InvalidProperty(format!("Invalid parameter format: {}", param))
                })?;

            parameters.insert(k.to_string(), v.to_string());
        }

        Ok(Property {
            name,
            parameters,
            value,
        })
    }
}
