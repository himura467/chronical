use super::datetime::ZonedDateTime;
use super::rfc9557::Rfc9557;
use super::rrule::RRule;
use super::rruleset_iter::RRuleSetIter;
use crate::error::RRuleError;

fn is_before(zdt: &ZonedDateTime, before: &ZonedDateTime, inclusive: bool) -> bool {
    if inclusive {
        zdt.datetime <= before.datetime
    } else {
        zdt.datetime < before.datetime
    }
}

fn is_after(zdt: &ZonedDateTime, after: &ZonedDateTime, inclusive: bool) -> bool {
    if inclusive {
        zdt.datetime >= after.datetime
    } else {
        zdt.datetime > after.datetime
    }
}

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

    pub fn try_into_iter(&self) -> Result<RRuleSetIter, RRuleError> {
        let rruleset = self.build()?;
        Ok(RRuleSetIter::new(rruleset.into_iter()))
    }

    pub fn all(&self) -> Result<Vec<String>, RRuleError> {
        self.try_into_iter()?
            .map(|result| {
                result
                    .map(|zdt| {
                        let rfc: Rfc9557 = zdt.into();
                        rfc.to_string()
                    })
                    .map_err(|e| RRuleError::ValidationError(e.to_string()))
            })
            .collect()
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
