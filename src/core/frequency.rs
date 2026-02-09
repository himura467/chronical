use crate::error::ParseError;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Frequency {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
    Minutely,
    Secondly,
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Yearly => "YEARLY",
            Self::Monthly => "MONTHLY",
            Self::Weekly => "WEEKLY",
            Self::Daily => "DAILY",
            Self::Hourly => "HOURLY",
            Self::Minutely => "MINUTELY",
            Self::Secondly => "SECONDLY",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Frequency {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "YEARLY" => Ok(Self::Yearly),
            "MONTHLY" => Ok(Self::Monthly),
            "WEEKLY" => Ok(Self::Weekly),
            "DAILY" => Ok(Self::Daily),
            "HOURLY" => Ok(Self::Hourly),
            "MINUTELY" => Ok(Self::Minutely),
            "SECONDLY" => Ok(Self::Secondly),
            _ => Err(ParseError::InvalidFrequency(s.to_string())),
        }
    }
}

impl From<Frequency> for rrule::Frequency {
    fn from(freq: Frequency) -> Self {
        match freq {
            Frequency::Yearly => Self::Yearly,
            Frequency::Monthly => Self::Monthly,
            Frequency::Weekly => Self::Weekly,
            Frequency::Daily => Self::Daily,
            Frequency::Hourly => Self::Hourly,
            Frequency::Minutely => Self::Minutely,
            Frequency::Secondly => Self::Secondly,
        }
    }
}

impl From<rrule::Frequency> for Frequency {
    fn from(freq: rrule::Frequency) -> Self {
        match freq {
            rrule::Frequency::Yearly => Self::Yearly,
            rrule::Frequency::Monthly => Self::Monthly,
            rrule::Frequency::Weekly => Self::Weekly,
            rrule::Frequency::Daily => Self::Daily,
            rrule::Frequency::Hourly => Self::Hourly,
            rrule::Frequency::Minutely => Self::Minutely,
            rrule::Frequency::Secondly => Self::Secondly,
        }
    }
}
