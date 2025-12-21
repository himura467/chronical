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
            Frequency::Yearly => core::frequency::Frequency::Yearly,
            Frequency::Monthly => core::frequency::Frequency::Monthly,
            Frequency::Weekly => core::frequency::Frequency::Weekly,
            Frequency::Daily => core::frequency::Frequency::Daily,
            Frequency::Hourly => core::frequency::Frequency::Hourly,
            Frequency::Minutely => core::frequency::Frequency::Minutely,
            Frequency::Secondly => core::frequency::Frequency::Secondly,
        }
    }
}

impl From<core::frequency::Frequency> for Frequency {
    fn from(freq: core::frequency::Frequency) -> Self {
        match freq {
            core::frequency::Frequency::Yearly => Frequency::Yearly,
            core::frequency::Frequency::Monthly => Frequency::Monthly,
            core::frequency::Frequency::Weekly => Frequency::Weekly,
            core::frequency::Frequency::Daily => Frequency::Daily,
            core::frequency::Frequency::Hourly => Frequency::Hourly,
            core::frequency::Frequency::Minutely => Frequency::Minutely,
            core::frequency::Frequency::Secondly => Frequency::Secondly,
        }
    }
}
