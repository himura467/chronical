use super::rfc9557::Rfc9557;
use super::rrule::RRule;
use super::rruleset_iter::RRuleSetIter;
use super::zoned_datetime::ZonedDateTime;
use crate::error::RRuleError;
use chrono::DateTime;

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

#[derive(Clone)]
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
        let rruleset = self.clone().build()?;
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

    pub fn between(
        &self,
        after: ZonedDateTime,
        before: ZonedDateTime,
        inclusive: Option<bool>,
    ) -> Result<Vec<String>, RRuleError> {
        let inclusive = inclusive.unwrap_or(false);

        self.try_into_iter()?
            .take_while(|result| {
                // Continue iterating until we reach or pass the upper bound
                match result {
                    Ok(zdt) => is_before(zdt, &before, inclusive),
                    Err(_) => true, // Continue on errors to collect them
                }
            })
            .filter_map(|result| match result {
                Ok(zdt) => {
                    if is_after(&zdt, &after, inclusive) {
                        let rfc: Rfc9557 = zdt.into();
                        Some(Ok(rfc.to_string()))
                    } else {
                        None
                    }
                }
                Err(e) => Some(Err(RRuleError::ValidationError(e.to_string()))),
            })
            .collect()
    }

    pub fn build(self) -> Result<rrule::RRuleSet, RRuleError> {
        let dt_start: DateTime<rrule::Tz> = self.dt_start.into();
        let mut builder = rrule::RRuleSet::new(dt_start);

        for rr in self.rrule {
            builder = builder.rrule(rr.build(dt_start)?);
        }
        for er in self.exrule {
            builder = builder.exrule(er.build(dt_start)?);
        }
        for rd in self.rdate {
            builder = builder.rdate(rd.into());
        }
        for ed in self.exdate {
            builder = builder.exdate(ed.into());
        }

        Ok(builder)
    }
}
