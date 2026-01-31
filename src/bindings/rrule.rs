use super::frequency::Frequency;
use crate::core;
use napi::bindgen_prelude::{Error, Result};
use napi_derive::napi;
use std::str::FromStr;

#[napi]
pub struct RRule {
    rrule: core::rrule::RRule,
}

#[napi]
impl RRule {
    #[napi(constructor)]
    pub fn new(freq: Frequency) -> Self {
        Self {
            rrule: core::rrule::RRule::new(freq.into()),
        }
    }

    #[napi(factory)]
    pub fn from_string(s: String) -> Result<Self> {
        let rrule =
            core::rrule::RRule::from_str(&s).map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Self { rrule })
    }

    #[napi(getter)]
    pub fn freq(&self) -> Frequency {
        self.rrule.freq.into()
    }

    #[napi(getter)]
    pub fn until(&self) -> Option<String> {
        self.rrule.until.clone().map(|zdt| {
            let rfc: core::rfc9557::Rfc9557 = zdt.into();
            rfc.to_string()
        })
    }

    #[napi(getter)]
    pub fn count(&self) -> Option<u32> {
        self.rrule.count
    }

    #[napi(getter)]
    pub fn interval(&self) -> Option<u16> {
        self.rrule.interval
    }

    #[napi(getter)]
    pub fn by_second(&self) -> Vec<u8> {
        self.rrule.by_second.clone()
    }

    #[napi(getter)]
    pub fn by_minute(&self) -> Vec<u8> {
        self.rrule.by_minute.clone()
    }

    #[napi(getter)]
    pub fn by_hour(&self) -> Vec<u8> {
        self.rrule.by_hour.clone()
    }

    #[napi(getter)]
    pub fn by_day(&self) -> Vec<String> {
        self.rrule.by_day.iter().map(|wn| wn.to_string()).collect()
    }

    #[napi(getter)]
    pub fn by_month_day(&self) -> Vec<i8> {
        self.rrule.by_month_day.clone()
    }

    #[napi(getter)]
    pub fn by_year_day(&self) -> Vec<i16> {
        self.rrule.by_year_day.clone()
    }

    #[napi(getter)]
    pub fn by_week_no(&self) -> Vec<i8> {
        self.rrule.by_week_no.clone()
    }

    #[napi(getter)]
    pub fn by_month(&self) -> Vec<u8> {
        self.rrule.by_month.clone()
    }

    #[napi(getter)]
    pub fn by_set_pos(&self) -> Vec<i32> {
        self.rrule.by_set_pos.clone()
    }

    #[napi(getter)]
    pub fn wkst(&self) -> Option<String> {
        self.rrule.wkst.map(|w| w.to_string())
    }
}
