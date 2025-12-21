use super::datetime::ZonedDateTime;
use super::frequency::Frequency;
use super::weekday::Weekday;
use super::weekday_num::WeekdayNum;

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
}
