use super::datetime::ZonedDateTime;
use super::frequency::Frequency;
use super::weekday::Weekday;
use super::weekday_num::WeekdayNum;
use crate::error::RRuleError;
use chrono::{DateTime, Month};
use chrono_tz::Tz;

pub struct RRule {
    pub freq: Frequency,
    pub until: Option<ZonedDateTime>,
    pub count: Option<u32>,
    pub interval: Option<u16>,
    pub by_second: Vec<u8>,
    pub by_minute: Vec<u8>,
    pub by_hour: Vec<u8>,
    pub by_day: Vec<WeekdayNum>,
    pub by_month_day: Vec<i8>,
    pub by_year_day: Vec<i16>,
    pub by_week_no: Vec<i8>,
    pub by_month: Vec<u8>,
    pub by_set_pos: Vec<i32>,
    pub wkst: Option<Weekday>,
}

impl RRule {
    pub fn new(freq: Frequency) -> Self {
        Self {
            freq,
            until: None,
            count: None,
            interval: None,
            by_second: Vec::new(),
            by_minute: Vec::new(),
            by_hour: Vec::new(),
            by_day: Vec::new(),
            by_month_day: Vec::new(),
            by_year_day: Vec::new(),
            by_week_no: Vec::new(),
            by_month: Vec::new(),
            by_set_pos: Vec::new(),
            wkst: None,
        }
    }

    pub fn build(
        &self,
        dt_start: DateTime<Tz>,
    ) -> Result<rrule::RRule<rrule::Validated>, RRuleError> {
        let mut builder = rrule::RRule::new(self.freq.into());

        if let Some(interval) = self.interval {
            builder = builder.interval(interval);
        }
        if let Some(count) = self.count {
            builder = builder.count(count);
        }
        if let Some(until) = &self.until {
            builder = builder.until(
                until
                    .datetime
                    .with_timezone(&rrule::Tz::Tz(until.datetime.timezone())),
            );
        }
        if let Some(wkst) = self.wkst {
            builder = builder.week_start(wkst.into());
        }
        builder = builder.by_set_pos(self.by_set_pos.clone());
        let months: Vec<Month> = self
            .by_month
            .iter()
            .map(|&m| {
                Month::try_from(m)
                    .map_err(|_| RRuleError::ValidationError(format!("Invalid month value: {}", m)))
            })
            .collect::<Result<Vec<Month>, RRuleError>>()?;
        builder = builder.by_month(&months);
        builder = builder.by_month_day(self.by_month_day.clone());
        builder = builder.by_year_day(self.by_year_day.clone());
        builder = builder.by_week_no(self.by_week_no.clone());
        builder = builder.by_weekday(self.by_day.iter().map(|&wn| wn.into()).collect());
        builder = builder.by_hour(self.by_hour.clone());
        builder = builder.by_minute(self.by_minute.clone());
        builder = builder.by_second(self.by_second.clone());

        builder
            .validate(dt_start.with_timezone(&rrule::Tz::Tz(dt_start.timezone())))
            .map_err(|e| RRuleError::ValidationError(e.to_string()))
    }
}
