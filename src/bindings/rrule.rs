use super::frequency::Frequency;
use crate::core;
use napi_derive::napi;

#[napi]
pub struct RRule {
    rrule: core::rrule::RRule,
}

#[napi]
impl RRule {
    #[napi(constructor)]
    pub fn new(freq: Frequency) -> Self {
        RRule {
            rrule: core::rrule::RRule::new(freq.into()),
        }
    }

    #[napi(getter)]
    pub fn freq(&self) -> Frequency {
        self.rrule.freq.into()
    }
}
