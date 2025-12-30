use super::datetime::ZonedDateTime;
use crate::error::ParseError;

pub struct RRuleSetIter {
    iter: rrule::RRuleSetIter,
}

impl RRuleSetIter {
    pub fn new(iter: rrule::RRuleSetIter) -> Self {
        Self { iter }
    }
}

impl Iterator for RRuleSetIter {
    type Item = Result<ZonedDateTime, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|dt| ZonedDateTime::try_from(dt))
    }
}
