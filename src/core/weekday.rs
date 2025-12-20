use crate::error::ParseError;
use std::str::FromStr;

pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl FromStr for Weekday {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MO" => Ok(Weekday::Mon),
            "TU" => Ok(Weekday::Tue),
            "WE" => Ok(Weekday::Wed),
            "TH" => Ok(Weekday::Thu),
            "FR" => Ok(Weekday::Fri),
            "SA" => Ok(Weekday::Sat),
            "SU" => Ok(Weekday::Sun),
            _ => Err(ParseError::InvalidWeekday(s.to_string())),
        }
    }
}

impl From<Weekday> for rrule::Weekday {
    fn from(wd: Weekday) -> Self {
        match wd {
            Weekday::Mon => rrule::Weekday::Mon,
            Weekday::Tue => rrule::Weekday::Tue,
            Weekday::Wed => rrule::Weekday::Wed,
            Weekday::Thu => rrule::Weekday::Thu,
            Weekday::Fri => rrule::Weekday::Fri,
            Weekday::Sat => rrule::Weekday::Sat,
            Weekday::Sun => rrule::Weekday::Sun,
        }
    }
}

impl From<rrule::Weekday> for Weekday {
    fn from(wd: rrule::Weekday) -> Self {
        match wd {
            rrule::Weekday::Mon => Weekday::Mon,
            rrule::Weekday::Tue => Weekday::Tue,
            rrule::Weekday::Wed => Weekday::Wed,
            rrule::Weekday::Thu => Weekday::Thu,
            rrule::Weekday::Fri => Weekday::Fri,
            rrule::Weekday::Sat => Weekday::Sat,
            rrule::Weekday::Sun => Weekday::Sun,
        }
    }
}
