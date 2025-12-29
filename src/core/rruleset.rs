use super::datetime::ZonedDateTime;
use super::rrule::RRule;
use crate::error::RRuleError;

pub struct RRuleSet {
    dt_start: ZonedDateTime,
    rrule: Vec<RRule>,
    rdate: Vec<ZonedDateTime>,
    exrule: Vec<RRule>,
    exdate: Vec<ZonedDateTime>,
}

impl RRuleSet {
    pub fn new(dt_start: ZonedDateTime) -> Self {
        Self {
            dt_start,
            rrule: Vec::new(),
            rdate: Vec::new(),
            exrule: Vec::new(),
            exdate: Vec::new(),
        }
    }

    pub fn all(&self) -> Vec<String> {
        vec![self.dt_start.to_rfc9557()]
    }

    pub fn build(&self) -> Result<rrule::RRuleSet, RRuleError> {
        let mut builder = rrule::RRuleSet::new((&self.dt_start).into());

        for rr in &self.rrule {
            builder = builder.rrule(rr.build(&self.dt_start)?);
        }
        for er in &self.exrule {
            builder = builder.exrule(er.build(&self.dt_start)?);
        }
        for rd in &self.rdate {
            builder = builder.rdate(rd.into());
        }
        for ed in &self.exdate {
            builder = builder.exdate(ed.into());
        }

        Ok(builder)
    }
}
