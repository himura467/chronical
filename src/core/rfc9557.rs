// TODO: Full RFC 9557 implementation
// See: https://datatracker.ietf.org/doc/html/rfc9557
//
// Format: YYYY-MM-DD T HH:mm:ss.sssssssss Z/±HH:mm [time_zone_id] [u-ca=calendar_id]

use crate::error::ParseError;
use chrono::DateTime;
use chrono_tz::Tz;
use std::fmt;
use std::str::FromStr;

pub struct Rfc9557 {
    pub datetime: DateTime<Tz>,
    pub calendar: Option<String>,
}

impl Rfc9557 {
    pub fn new(datetime: DateTime<Tz>, calendar: Option<String>) -> Self {
        Self { datetime, calendar }
    }
}

impl fmt::Display for Rfc9557 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = format!(
            "{}[{}]",
            self.datetime.to_rfc3339(),
            self.datetime.timezone()
        );

        if let Some(c) = &self.calendar {
            write!(f, "{}[u-ca={}]", base, c)
        } else {
            write!(f, "{}", base)
        }
    }
}

impl FromStr for Rfc9557 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('[').collect();

        if parts.is_empty() {
            return Err(ParseError::InvalidDateTime(s.to_string()));
        }

        let datetime_str = parts[0];
        let mut timezone = chrono_tz::UTC;
        let mut calendar = None;

        for part in &parts[1..] {
            let part = part.trim_end_matches(']');
            if part.starts_with("u-ca=") {
                calendar = part.strip_prefix("u-ca=").map(|s| s.to_string());
            } else {
                timezone = part
                    .parse::<Tz>()
                    .map_err(|_| ParseError::InvalidDateTime(s.to_string()))?;
            }
        }

        let datetime = DateTime::parse_from_rfc3339(datetime_str)
            .map_err(|_| ParseError::InvalidDateTime(s.to_string()))?;

        let zoned_dt = datetime.with_timezone(&timezone);

        Ok(Self {
            datetime: zoned_dt,
            calendar,
        })
    }
}
