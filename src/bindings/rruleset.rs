use crate::core;
use napi::bindgen_prelude::{Error, Result};
use napi_derive::napi;
use std::str::FromStr;

#[napi]
pub struct RRuleSet {
    rruleset: core::rruleset::RRuleSet,
}

#[napi]
impl RRuleSet {
    #[napi(constructor)]
    pub fn new(dt_start: String) -> Result<Self> {
        let zoned_dt = core::datetime::ZonedDateTime::from_str(&dt_start)
            .map_err(|e| Error::from_reason(e.to_string()))?;

        Ok(RRuleSet {
            rruleset: core::rruleset::RRuleSet::new(zoned_dt),
        })
    }

    #[napi]
    pub fn all(&self) -> Vec<String> {
        self.rruleset.all()
    }
}
