use super::dtstart::DtStart;
use super::properties::Properties;
use super::rdate::RDate;
use super::rfc9557::Rfc9557;
use super::rrule::RRule;
use super::rruleset_iter::RRuleSetIter;
use super::zoned_datetime::ZonedDateTime;
use crate::error::{ParseError, RRuleError};
use chrono::DateTime;
use std::str::FromStr;

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
    dtstart: DtStart,
    rrule: Vec<RRule>,
    rdate: Vec<RDate>,
    exrule: Vec<RRule>,
    exdate: Vec<RDate>,
}

impl RRuleSet {
    pub fn new(dtstart: DtStart) -> Self {
        Self {
            dtstart,
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
        let dtstart: DateTime<rrule::Tz> = self.dtstart.dtstart.into();
        let mut builder = rrule::RRuleSet::new(dtstart);

        for rr in self.rrule {
            builder = builder.rrule(rr.build(dtstart)?);
        }
        for er in self.exrule {
            builder = builder.exrule(er.build(dtstart)?);
        }
        for rd in self.rdate {
            let tzid = rd.tzid.as_deref();
            for value in rd.values {
                let zdt = value
                    .to_zoned_datetime(tzid)
                    .map_err(|e| RRuleError::ValidationError(e.to_string()))?;
                builder = builder.rdate(zdt.into());
            }
        }
        for ed in self.exdate {
            let tzid = ed.tzid.as_deref();
            for value in ed.values {
                let zdt = value
                    .to_zoned_datetime(tzid)
                    .map_err(|e| RRuleError::ValidationError(e.to_string()))?;
                builder = builder.exdate(zdt.into());
            }
        }

        Ok(builder)
    }
}

impl FromStr for RRuleSet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let properties: Properties = s.parse()?;

        let mut dtstart = None;
        let mut rrule = Vec::new();
        let mut rdate = Vec::new();
        let mut exrule = Vec::new();
        let mut exdate = Vec::new();

        for property in properties.properties {
            match property.name.as_str() {
                "DTSTART" => {
                    dtstart = Some(DtStart::try_from(property)?);
                }
                "RRULE" => {
                    rrule.push(RRule::try_from(property)?);
                }
                "RDATE" => {
                    rdate.push(RDate::try_from(property)?);
                }
                "EXRULE" => {
                    exrule.push(RRule::try_from(property)?);
                }
                "EXDATE" => {
                    exdate.push(RDate::try_from(property)?);
                }
                _ => {
                    return Err(ParseError::InvalidProperty(format!(
                        "Unexpected property in RRuleSet: {}",
                        property.name
                    )));
                }
            }
        }
        let dtstart = dtstart
            .ok_or_else(|| ParseError::InvalidProperty("Missing DTSTART property".to_string()))?;

        Ok(Self {
            dtstart,
            rrule,
            rdate,
            exrule,
            exdate,
        })
    }
}
