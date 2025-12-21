// TODO: Full RFC 9557 implementation
// See: https://datatracker.ietf.org/doc/html/rfc9557
//
// This is a minimal implementation to support conversion between
// Rust DateTime and JavaScript Temporal.ZonedDateTime.
//
// Format: YYYY-MM-DD T HH:mm:ss.sssssssss Z/±HH:mm [time_zone_id] [u-ca=calendar_id]

use crate::error::ParseError;
use chrono::DateTime;
use chrono_tz::Tz;

pub fn to_rfc9557(datetime: &DateTime<Tz>, calendar: &Option<String>) -> String {
    let base = format!("{}[{}]", datetime.to_rfc3339(), datetime.timezone());

    if let Some(c) = calendar {
        format!("{}[u-ca={}]", base, c)
    } else {
        base
    }
}

pub fn from_rfc9557(s: &str) -> Result<(DateTime<Tz>, Option<String>), ParseError> {
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

    Ok((zoned_dt, calendar))
}
