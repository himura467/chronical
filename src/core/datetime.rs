use super::zoned_datetime::ZonedDateTime;
use crate::error::ParseError;
use chrono::{NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::{Tz, UTC};
use std::fmt;
use std::str::FromStr;

/// Parses an iCalendar DATE-TIME string (`YYYYMMDDTHHmmss`), clamping any
/// leap second (second=60) to 59 per RFC 5545:
/// Implementations that do not support leap seconds SHOULD interpret the second 60 as equivalent to the second 59.
fn parse_datetime(s: &str) -> Result<NaiveDateTime, ParseError> {
    let clamped;
    let dt_str = if s.len() == 15 && s.get(13..15) == Some("60") {
        clamped = format!("{}59", &s[..13]);
        &clamped
    } else {
        s
    };
    NaiveDateTime::parse_from_str(dt_str, "%Y%m%dT%H%M%S")
        .map_err(|_| ParseError::InvalidDateTime(s.to_string()))
}

/// Represents a parsed iCalendar DATE or DATE-TIME value
///
/// Variants correspond to the three iCalendar temporal formats:
/// - `Date` — `YYYYMMDD`
/// - `Local` — `YYYYMMDDTHHmmss` (requires TZID for timezone resolution)
/// - `Utc` — `YYYYMMDDTHHmmssZ`
#[derive(Clone)]
pub enum ICalDateTime {
    /// DATE value (YYYYMMDD)
    Date(NaiveDate),
    /// Local DATE-TIME value (YYYYMMDDTHHmmss)
    Local(NaiveDateTime),
    /// UTC DATE-TIME value (YYYYMMDDTHHmmssZ)
    Utc(NaiveDateTime),
}

impl ICalDateTime {
    /// Resolves to a [`ZonedDateTime`] using the given IANA timezone ID
    ///
    /// - `Date` values are interpreted as midnight; uses `tzid` if provided, otherwise UTC.
    /// - `Local` values require a `tzid` to resolve the timezone.
    /// - `Utc` values are always UTC; providing a `tzid` is an error.
    pub fn to_zoned_datetime(&self, tzid: Option<&str>) -> Result<ZonedDateTime, ParseError> {
        let (naive, tz) = match self {
            Self::Date(date) => {
                let tz = match tzid {
                    Some(tz_str) => tz_str
                        .parse::<Tz>()
                        .map_err(|_| ParseError::InvalidTimezone(tz_str.to_string()))?,
                    None => UTC,
                };
                (
                    date.and_hms_opt(0, 0, 0)
                        .expect("00:00:00 is always a valid time"),
                    tz,
                )
            }
            Self::Local(naive) => {
                let tz_str = tzid.ok_or_else(|| {
                    ParseError::InvalidDateTime(format!(
                        "Missing TZID for local datetime: {}",
                        naive.format("%Y%m%dT%H%M%S")
                    ))
                })?;
                let tz: Tz = tz_str
                    .parse()
                    .map_err(|_| ParseError::InvalidTimezone(tz_str.to_string()))?;
                (*naive, tz)
            }
            Self::Utc(naive) => {
                if tzid.is_some() {
                    return Err(ParseError::InvalidDateTime(format!(
                        "UTC DATE-TIME cannot have a TZID: {}Z",
                        naive.format("%Y%m%dT%H%M%S")
                    )));
                }
                (*naive, UTC)
            }
        };

        let datetime = tz.from_local_datetime(&naive).single().ok_or_else(|| {
            ParseError::InvalidDateTime(format!(
                "Ambiguous datetime: {}",
                naive.format("%Y%m%dT%H%M%S")
            ))
        })?;

        Ok(ZonedDateTime::new(datetime, None))
    }
}

impl fmt::Display for ICalDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date(nd) => write!(f, "{}", nd.format("%Y%m%d")),
            Self::Local(ndt) => write!(f, "{}", ndt.format("%Y%m%dT%H%M%S")),
            Self::Utc(ndt) => write!(f, "{}Z", ndt.format("%Y%m%dT%H%M%S")),
        }
    }
}

impl FromStr for ICalDateTime {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_suffix('Z') {
            Ok(Self::Utc(parse_datetime(rest)?))
        } else if s.contains('T') {
            Ok(Self::Local(parse_datetime(s)?))
        } else {
            let date = NaiveDate::parse_from_str(s, "%Y%m%d")
                .map_err(|_| ParseError::InvalidDateTime(s.to_string()))?;
            Ok(Self::Date(date))
        }
    }
}
