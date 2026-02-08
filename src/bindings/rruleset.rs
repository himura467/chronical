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
        let zdt = core::zoned_datetime::ZonedDateTime::from_str(&dt_start)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let dtstart = core::dtstart::DtStart::new(zdt);

        Ok(Self {
            rruleset: core::rruleset::RRuleSet::new(dtstart),
        })
    }

    #[napi(factory)]
    pub fn from_string(s: String) -> Result<Self> {
        let rruleset = core::rruleset::RRuleSet::from_str(&s)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(Self { rruleset })
    }

    #[napi(getter)]
    pub fn dt_start(&self) -> String {
        core::rfc9557::Rfc9557::from(&self.rruleset.dtstart().dtstart).to_string()
    }

    #[napi]
    pub fn all(&self) -> Result<Vec<String>> {
        self.rruleset
            .all()
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    #[napi]
    pub fn between(
        &self,
        after: String,
        before: String,
        inclusive: Option<bool>,
    ) -> Result<Vec<String>> {
        let after_zdt = core::zoned_datetime::ZonedDateTime::from_str(&after)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        let before_zdt = core::zoned_datetime::ZonedDateTime::from_str(&before)
            .map_err(|e| Error::from_reason(e.to_string()))?;

        self.rruleset
            .between(after_zdt, before_zdt, inclusive)
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}
