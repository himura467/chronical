use super::datetime::ICalDateTime;
use super::pairs::Pairs;
use super::property::{Property, Value};
use crate::error::ParseError;

/// Represents the EXDATE property from iCalendar
///
/// EXDATE defines the list of DATE-TIME exceptions for recurring events,
/// to-dos, journal entries, or time zone definitions
#[derive(Clone)]
pub struct ExDate {
    pub values: Vec<ICalDateTime>,
    pub tzid: Option<String>,
    pub value_type: Option<String>,
}

impl From<&ExDate> for Property {
    fn from(exdate: &ExDate) -> Self {
        let mut parameters = Pairs::new();
        if let Some(tzid) = &exdate.tzid {
            parameters.insert("TZID".to_string(), tzid.clone());
        }
        if let Some(value_type) = &exdate.value_type {
            parameters.insert("VALUE".to_string(), value_type.clone());
        }
        let value_str = exdate
            .values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        Property::new("EXDATE".to_string(), parameters, Value::Single(value_str))
    }
}

impl TryFrom<Property> for ExDate {
    type Error = ParseError;

    fn try_from(property: Property) -> Result<Self, Self::Error> {
        if property.name != "EXDATE" {
            return Err(ParseError::InvalidProperty(format!(
                "Expected EXDATE property, got {}",
                property.name
            )));
        }

        let tzid = property.parameters.get("TZID").cloned();
        let value_type = property.parameters.get("VALUE").cloned();
        let values = match &property.value {
            Value::Single(s) => s
                .split(',')
                .map(|v| v.parse::<ICalDateTime>())
                .collect::<Result<Vec<ICalDateTime>, ParseError>>()?,
            Value::Pairs(_) => {
                return Err(ParseError::InvalidProperty(
                    "EXDATE value must be a list of DATE-TIME values".to_string(),
                ));
            }
        };

        Ok(Self {
            values,
            tzid,
            value_type,
        })
    }
}
