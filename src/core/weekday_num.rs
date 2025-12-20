use crate::core::weekday::Weekday;
use crate::error::ParseError;
use std::str::FromStr;

pub enum WeekdayNum {
    Every(Weekday),
    Nth(i16, Weekday),
}

impl WeekdayNum {
    pub fn new(number: Option<i16>, weekday: Weekday) -> Self {
        match number {
            Some(n) => WeekdayNum::Nth(n, weekday),
            None => WeekdayNum::Every(weekday),
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
    s[..s.len() - 2].parse::<i16>()
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

        Ok(WeekdayNum::new(n, weekday))
    }
}

impl From<WeekdayNum> for rrule::NWeekday {
    fn from(wdn: WeekdayNum) -> Self {
        match wdn {
            WeekdayNum::Every(wd) => rrule::NWeekday::Every(wd.into()),
            WeekdayNum::Nth(n, wd) => rrule::NWeekday::Nth(n, wd.into()),
        }
    }
}

impl From<rrule::NWeekday> for WeekdayNum {
    fn from(nwd: rrule::NWeekday) -> Self {
        match nwd {
            rrule::NWeekday::Every(wd) => WeekdayNum::Every(wd.into()),
            rrule::NWeekday::Nth(n, wd) => WeekdayNum::Nth(n, wd.into()),
        }
    }
}
