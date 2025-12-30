use super::rfc9557::Rfc9557;
use crate::error::ParseError;
use chrono::DateTime;
use chrono_tz::Tz;
use std::str::FromStr;

pub struct ZonedDateTime {
    pub datetime: DateTime<Tz>,
    pub calendar: Option<String>,
}

impl ZonedDateTime {
    pub fn new(datetime: DateTime<Tz>, calendar: Option<String>) -> Self {
        Self { datetime, calendar }
    }
}

impl From<ZonedDateTime> for Rfc9557 {
    fn from(zdt: ZonedDateTime) -> Self {
        Rfc9557::new(zdt.datetime, zdt.calendar)
    }
}

impl From<Rfc9557> for ZonedDateTime {
    fn from(rfc: Rfc9557) -> Self {
        Self {
            datetime: rfc.datetime,
            calendar: rfc.calendar,
        }
    }
}

impl FromStr for ZonedDateTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Rfc9557::from_str(s).map(ZonedDateTime::from)
    }
}

impl From<&ZonedDateTime> for DateTime<rrule::Tz> {
    fn from(zdt: &ZonedDateTime) -> Self {
        zdt.datetime
            .with_timezone(&rrule::Tz::Tz(zdt.datetime.timezone()))
    }
}

impl TryFrom<DateTime<rrule::Tz>> for ZonedDateTime {
    type Error = ParseError;

    fn try_from(datetime: DateTime<rrule::Tz>) -> Result<Self, Self::Error> {
        let chrono_tz = match datetime.timezone() {
            rrule::Tz::Tz(tz) => tz,
            rrule::Tz::Local(_) => {
                return Err(ParseError::InvalidTimezone(
                    "Local timezone is not supported in ZonedDateTime".to_string(),
                ));
            }
        };
        let chrono_datetime = datetime.with_timezone(&chrono_tz);
        Ok(Self {
            datetime: chrono_datetime,
            calendar: None,
        })
    }
}
