use super::datetime::ZonedDateTime;
use super::frequency::Frequency;
use super::property::{Property, Value};
use super::weekday::Weekday;
use super::weekday_num::WeekdayNum;
use crate::error::{ParseError, RRuleError};
use chrono::Month;

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
        dt_start: &ZonedDateTime,
    ) -> Result<rrule::RRule<rrule::Validated>, RRuleError> {
        let mut builder = rrule::RRule::new(self.freq.into());

        if let Some(interval) = self.interval {
            builder = builder.interval(interval);
        }
        if let Some(count) = self.count {
            builder = builder.count(count);
        }
        if let Some(until) = &self.until {
            builder = builder.until(until.into());
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
            .validate(dt_start.into())
            .map_err(|e| RRuleError::ValidationError(e.to_string()))
    }
}

impl TryFrom<Property> for RRule {
    type Error = ParseError;

    fn try_from(property: Property) -> Result<Self, Self::Error> {
        if property.name != "RRULE" && property.name != "EXRULE" {
            return Err(ParseError::InvalidProperty(format!(
                "Expected RRULE or EXRULE property, got {}",
                property.name
            )));
        }

        let pairs = match property.value {
            Value::Pairs(p) => p,
            Value::Single(s) => {
                return Err(ParseError::InvalidProperty(format!(
                    "RRULE value must be key-value pairs, got: {}",
                    s
                )));
            }
        };

        let freq: Frequency = pairs
            .get_parsed("FREQ")?
            .ok_or_else(|| ParseError::InvalidProperty("RRULE must have FREQ".to_string()))?;
        let until = pairs.get_parsed("UNTIL")?;
        let count = pairs
            .get_parsed::<u32, _>("COUNT")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid COUNT: {}", e)))?;
        let interval = pairs
            .get_parsed::<u16, _>("INTERVAL")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid INTERVAL: {}", e)))?;
        let by_second = pairs
            .get_csv::<u8, _>("BYSECOND")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYSECOND: {}", e)))?
            .unwrap_or_default();
        let by_minute = pairs
            .get_csv::<u8, _>("BYMINUTE")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYMINUTE: {}", e)))?
            .unwrap_or_default();
        let by_hour = pairs
            .get_csv::<u8, _>("BYHOUR")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYHOUR: {}", e)))?
            .unwrap_or_default();
        let by_day = pairs.get_csv("BYDAY")?.unwrap_or_default();
        let by_month_day = pairs
            .get_csv::<i8, _>("BYMONTHDAY")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYMONTHDAY: {}", e)))?
            .unwrap_or_default();
        let by_year_day = pairs
            .get_csv::<i16, _>("BYYEARDAY")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYYEARDAY: {}", e)))?
            .unwrap_or_default();
        let by_week_no = pairs
            .get_csv::<i8, _>("BYWEEKNO")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYWEEKNO: {}", e)))?
            .unwrap_or_default();
        let by_month = pairs
            .get_csv::<u8, _>("BYMONTH")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYMONTH: {}", e)))?
            .unwrap_or_default();
        let by_set_pos = pairs
            .get_csv::<i32, _>("BYSETPOS")
            .map_err(|e| ParseError::InvalidProperty(format!("Invalid BYSETPOS: {}", e)))?
            .unwrap_or_default();
        let wkst = pairs.get_parsed("WKST")?;

        Ok(RRule {
            freq,
            until,
            count,
            interval,
            by_second,
            by_minute,
            by_hour,
            by_day,
            by_month_day,
            by_year_day,
            by_week_no,
            by_month,
            by_set_pos,
            wkst,
        })
    }
}
