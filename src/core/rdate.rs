use super::datetime::ICalDateTime;
use super::pairs::Pairs;
use super::property::{Property, Value};
use crate::error::ParseError;

/// Represents the RDATE property from iCalendar
///
/// RDATE defines the list of DATE-TIME values for recurring events,
/// to-dos, journal entries, or time zone definitions
#[derive(Clone)]
pub struct RDate {
    pub values: Vec<ICalDateTime>,
    pub tzid: Option<String>,
    pub value_type: Option<String>,
}

impl From<&RDate> for Property {
    fn from(rdate: &RDate) -> Self {
        let mut parameters = Pairs::new();
        if let Some(tzid) = &rdate.tzid {
            parameters.insert("TZID".to_string(), tzid.clone());
        }
        if let Some(value_type) = &rdate.value_type {
            parameters.insert("VALUE".to_string(), value_type.clone());
        }
        let value_str = rdate
            .values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        Property::new("RDATE".to_string(), parameters, Value::Single(value_str))
    }
}

impl TryFrom<Property> for RDate {
    type Error = ParseError;

    fn try_from(property: Property) -> Result<Self, Self::Error> {
        if property.name != "RDATE" {
            return Err(ParseError::InvalidProperty(format!(
                "Expected RDATE property, got {}",
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
                    "RDATE value must be a list of DATE-TIME values".to_string(),
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
