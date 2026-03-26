use super::datetime::ICalDateTime;
use super::pairs::Pairs;
use super::property::{Property, Value};
use super::zoned_datetime::ZonedDateTime;
use crate::error::ParseError;

/// Represents the DTSTART property from iCalendar
///
/// The DTSTART property specifies when the recurrence rule pattern begins
#[derive(Clone)]
pub struct DtStart {
    pub dtstart: ZonedDateTime,
}

impl DtStart {
    pub fn new(dtstart: ZonedDateTime) -> Self {
        Self { dtstart }
    }
}

impl From<&DtStart> for Property {
    fn from(dtstart: &DtStart) -> Self {
        let datetime = &dtstart.dtstart.datetime;
        let tz = datetime.timezone();
        let mut parameters = Pairs::new();
        let value = if tz.name() == "UTC" {
            Value::Single(datetime.format("%Y%m%dT%H%M%SZ").to_string())
        } else {
            parameters.insert("TZID".to_string(), tz.name().to_string());
            Value::Single(datetime.format("%Y%m%dT%H%M%S").to_string())
        };
        Property::new("DTSTART".to_string(), parameters, value)
    }
}

impl TryFrom<Property> for DtStart {
    type Error = ParseError;

    fn try_from(property: Property) -> Result<Self, Self::Error> {
        if property.name != "DTSTART" {
            return Err(ParseError::InvalidProperty(format!(
                "Expected DTSTART property, got {}",
                property.name
            )));
        }

        let tzid = property.parameters.get("TZID").map(|s| s.as_str());
        let dtstart = match &property.value {
            Value::Single(s) => {
                let dt: ICalDateTime = s.parse()?;
                dt.to_zoned_datetime(tzid)?
            }
            Value::Pairs(_) => {
                return Err(ParseError::InvalidProperty(
                    "DTSTART value must be a DATE-TIME".to_string(),
                ));
            }
        };

        Ok(Self { dtstart })
    }
}
