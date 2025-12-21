use crate::error::ParseError;
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
            Frequency::Yearly => rrule::Frequency::Yearly,
            Frequency::Monthly => rrule::Frequency::Monthly,
            Frequency::Weekly => rrule::Frequency::Weekly,
            Frequency::Daily => rrule::Frequency::Daily,
            Frequency::Hourly => rrule::Frequency::Hourly,
            Frequency::Minutely => rrule::Frequency::Minutely,
            Frequency::Secondly => rrule::Frequency::Secondly,
        }
    }
}

impl From<rrule::Frequency> for Frequency {
    fn from(freq: rrule::Frequency) -> Self {
        match freq {
            rrule::Frequency::Yearly => Frequency::Yearly,
            rrule::Frequency::Monthly => Frequency::Monthly,
            rrule::Frequency::Weekly => Frequency::Weekly,
            rrule::Frequency::Daily => Frequency::Daily,
            rrule::Frequency::Hourly => Frequency::Hourly,
            rrule::Frequency::Minutely => Frequency::Minutely,
            rrule::Frequency::Secondly => Frequency::Secondly,
        }
    }
}
