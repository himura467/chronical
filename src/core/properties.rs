use super::property::Property;
use crate::error::ParseError;
use std::str::FromStr;

/// A collection of iCalendar properties
///
/// Converts a newline-delimited string of iCalendar properties into a list of parsed [`Property`] objects
///
/// Example:
/// ```text
/// DTSTART;TZID=America/New_York:19970105T083000
/// RRULE:FREQ=YEARLY;INTERVAL=2;BYMONTH=1;BYDAY=SU;BYHOUR=8,9;BYMINUTE=30
/// ```
pub struct Properties {
    pub properties: Vec<Property>,
}

impl Properties {
    pub fn new(properties: Vec<Property>) -> Self {
        Self { properties }
    }
}

impl FromStr for Properties {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let properties = s
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.parse::<Property>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { properties })
    }
}
