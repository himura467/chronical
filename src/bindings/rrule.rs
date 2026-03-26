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
    pub fn new(freq: Frequency) -> Result<Self> {
        Ok(Self {
            rrule: core::rrule::RRule::new(freq.into()),
        })
    }

    #[napi(factory)]
    pub fn from_string(s: String) -> Result<Self> {
        let rrule =
            core::rrule::RRule::from_str(&s).map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Self { rrule })
    }

    #[napi]
    pub fn to_string(&self) -> Result<String> {
        Ok(self.rrule.to_string())
    }

    #[napi(getter)]
    pub fn freq(&self) -> Result<Frequency> {
        Ok(self.rrule.freq.into())
    }

    #[napi(getter)]
    pub fn until(&self) -> Result<Option<String>> {
        Ok(self.rrule.until.clone().map(|zdt| {
            let rfc: core::rfc9557::Rfc9557 = zdt.into();
            rfc.to_string()
        }))
    }

    #[napi(getter)]
    pub fn count(&self) -> Result<Option<u32>> {
        Ok(self.rrule.count)
    }

    #[napi(getter)]
    pub fn interval(&self) -> Result<Option<u16>> {
        Ok(self.rrule.interval)
    }

    #[napi(getter)]
    pub fn by_second(&self) -> Result<Vec<u8>> {
        Ok(self.rrule.by_second.clone())
    }

    #[napi(getter)]
    pub fn by_minute(&self) -> Result<Vec<u8>> {
        Ok(self.rrule.by_minute.clone())
    }

    #[napi(getter)]
    pub fn by_hour(&self) -> Result<Vec<u8>> {
        Ok(self.rrule.by_hour.clone())
    }

    #[napi(getter)]
    pub fn by_day(&self) -> Result<Vec<String>> {
        Ok(self.rrule.by_day.iter().map(|wn| wn.to_string()).collect())
    }

    #[napi(getter)]
    pub fn by_month_day(&self) -> Result<Vec<i8>> {
        Ok(self.rrule.by_month_day.clone())
    }

    #[napi(getter)]
    pub fn by_year_day(&self) -> Result<Vec<i16>> {
        Ok(self.rrule.by_year_day.clone())
    }

    #[napi(getter)]
    pub fn by_week_no(&self) -> Result<Vec<i8>> {
        Ok(self.rrule.by_week_no.clone())
    }

    #[napi(getter)]
    pub fn by_month(&self) -> Result<Vec<u8>> {
        Ok(self.rrule.by_month.clone())
    }

    #[napi(getter)]
    pub fn by_set_pos(&self) -> Result<Vec<i32>> {
        Ok(self.rrule.by_set_pos.clone())
    }

    #[napi(getter)]
    pub fn wkst(&self) -> Result<Option<String>> {
        Ok(self.rrule.wkst.map(|w| w.to_string()))
    }
}
