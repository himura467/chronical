use crate::core;
use napi_derive::napi;

#[napi(string_enum)]
pub enum Frequency {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
    Minutely,
    Secondly,
}

impl From<Frequency> for core::frequency::Frequency {
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

impl From<core::frequency::Frequency> for Frequency {
    fn from(freq: core::frequency::Frequency) -> Self {
        match freq {
            core::frequency::Frequency::Yearly => Self::Yearly,
            core::frequency::Frequency::Monthly => Self::Monthly,
            core::frequency::Frequency::Weekly => Self::Weekly,
            core::frequency::Frequency::Daily => Self::Daily,
            core::frequency::Frequency::Hourly => Self::Hourly,
            core::frequency::Frequency::Minutely => Self::Minutely,
            core::frequency::Frequency::Secondly => Self::Secondly,
        }
    }
}
