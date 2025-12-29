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

impl FromStr for ZonedDateTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (datetime, calendar) = rfc9557::from_rfc9557(s)?;
        Ok(ZonedDateTime { datetime, calendar })
    }
}

impl From<&ZonedDateTime> for DateTime<rrule::Tz> {
    fn from(zdt: &ZonedDateTime) -> Self {
        zdt.datetime
            .with_timezone(&rrule::Tz::Tz(zdt.datetime.timezone()))
    }
}
