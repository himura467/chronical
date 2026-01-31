use crate::error::ParseError;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Mon => "MO",
            Self::Tue => "TU",
            Self::Wed => "WE",
            Self::Thu => "TH",
            Self::Fri => "FR",
            Self::Sat => "SA",
            Self::Sun => "SU",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Weekday {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "MO" => Ok(Self::Mon),
            "TU" => Ok(Self::Tue),
            "WE" => Ok(Self::Wed),
            "TH" => Ok(Self::Thu),
            "FR" => Ok(Self::Fri),
            "SA" => Ok(Self::Sat),
            "SU" => Ok(Self::Sun),
            _ => Err(ParseError::InvalidWeekday(s.to_string())),
        }
    }
}

impl From<Weekday> for rrule::Weekday {
    fn from(w: Weekday) -> Self {
        match w {
            Weekday::Mon => Self::Mon,
            Weekday::Tue => Self::Tue,
            Weekday::Wed => Self::Wed,
            Weekday::Thu => Self::Thu,
            Weekday::Fri => Self::Fri,
            Weekday::Sat => Self::Sat,
            Weekday::Sun => Self::Sun,
        }
    }
}

impl From<rrule::Weekday> for Weekday {
    fn from(w: rrule::Weekday) -> Self {
        match w {
            rrule::Weekday::Mon => Self::Mon,
            rrule::Weekday::Tue => Self::Tue,
            rrule::Weekday::Wed => Self::Wed,
            rrule::Weekday::Thu => Self::Thu,
            rrule::Weekday::Fri => Self::Fri,
            rrule::Weekday::Sat => Self::Sat,
            rrule::Weekday::Sun => Self::Sun,
        }
    }
}
