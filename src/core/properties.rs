use super::property::Property;
use crate::error::ParseError;
use std::fmt;
use std::str::FromStr;

/// A collection of iCalendar properties
///
/// Converts a newline-delimited string of iCalendar properties into a list of parsed [`Property`] objects
///
/// Example:
/// ```text
/// DTSTART;TZID=America/New_York:19970105T083000
/// RRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;
///  BYMINUTE=30
/// ```
pub struct Properties {
    pub properties: Vec<Property>,
}

impl Properties {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }

    pub fn push(mut self, property: Property) -> Self {
        self.properties.push(property);
        self
    }
}

impl fmt::Display for Properties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for property in &self.properties {
            write!(f, "{}\r\n", property)?;
        }
        Ok(())
    }
}

impl FromStr for Properties {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Unfold continuation lines per RFC 5545:
        // Any sequence of CRLF followed immediately by a single linear white-space character
        // is ignored (i.e., removed) when processing the content type.
        let mut unfolded = String::new();
        for line in s.lines() {
            if let Some(rest) = line.strip_prefix(' ').or_else(|| line.strip_prefix('\t')) {
                unfolded.push_str(rest);
            } else {
                if !unfolded.is_empty() {
                    unfolded.push('\n');
                }
                unfolded.push_str(line);
            }
        }

        let properties = unfolded
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.parse::<Property>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { properties })
    }
}
