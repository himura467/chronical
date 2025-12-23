use super::datetime::ZonedDateTime;
use super::rrule::RRule;

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
}
