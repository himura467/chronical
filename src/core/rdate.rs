use super::datetime::DateTime;
use super::property::{Property, Value};
use crate::error::ParseError;

/// Represents the RDATE or EXDATE property from iCalendar
///
/// RDATE defines the list of DATE-TIME values for recurring events,
/// to-dos, journal entries, or time zone definitions
///
/// EXDATE defines the list of DATE-TIME exceptions for recurring events,
/// to-dos, journal entries, or time zone definitions
pub struct RDate {
    pub values: Vec<DateTime>,
    pub tzid: Option<String>,
    pub value_type: Option<String>,
}

impl TryFrom<Property> for RDate {
    type Error = ParseError;

    fn try_from(property: Property) -> Result<Self, Self::Error> {
        if property.name != "RDATE" && property.name != "EXDATE" {
            return Err(ParseError::InvalidProperty(format!(
                "Expected RDATE or EXDATE property, got {}",
                property.name
            )));
        }

        let tzid = property.parameters.get("TZID").cloned();
        let value_type = property.parameters.get("VALUE").cloned();
        let values = match &property.value {
            Value::Single(s) => s
                .split(',')
                .map(|v| v.parse::<DateTime>())
                .collect::<Result<Vec<DateTime>, ParseError>>()?,
            Value::Pairs(_) => {
                return Err(ParseError::InvalidProperty(format!(
                    "{} value must be a list of DATE-TIME values",
                    property.name
                )));
            }
        };

        Ok(Self {
            values,
            tzid,
            value_type,
        })
    }
}
