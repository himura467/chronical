use super::weekday::Weekday;
use crate::error::ParseError;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum WeekdayNum {
    Every(Weekday),
    Nth(i16, Weekday),
}

impl WeekdayNum {
    pub fn new(number: Option<i16>, weekday: Weekday) -> Self {
        match number {
            Some(n) => Self::Nth(n, weekday),
            None => Self::Every(weekday),
        }
    }
}

impl fmt::Display for WeekdayNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Every(w) => write!(f, "{}", w),
            Self::Nth(n, w) => write!(f, "{}{}", n, w),
        }
    }
}

fn extract_weekday(s: &str) -> Result<Weekday, ParseError> {
    if s.len() < 2 {
        return Err(ParseError::InvalidWeekday(s.to_string()));
    }

    s[s.len() - 2..].parse()
}

fn extract_number(s: &str) -> Result<i16, ParseError> {
    s[..s.len() - 2]
        .parse::<i16>()
        .map_err(|_| ParseError::InvalidWeekday(s.to_string()))
}

impl FromStr for WeekdayNum {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let weekday = extract_weekday(s)?;
        let n = if s.len() > 2 {
            Some(extract_number(s)?)
        } else {
            None
        };

        Ok(Self::new(n, weekday))
    }
}

impl From<WeekdayNum> for rrule::NWeekday {
    fn from(wn: WeekdayNum) -> Self {
        match wn {
            WeekdayNum::Every(w) => Self::Every(w.into()),
            WeekdayNum::Nth(n, w) => Self::Nth(n, w.into()),
        }
    }
}

impl From<rrule::NWeekday> for WeekdayNum {
    fn from(nw: rrule::NWeekday) -> Self {
        match nw {
            rrule::NWeekday::Every(w) => Self::Every(w.into()),
            rrule::NWeekday::Nth(n, w) => Self::Nth(n, w.into()),
        }
    }
}
