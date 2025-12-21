use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("`{0}` is not a valid datetime.")]
    InvalidDateTime(String),
    #[error("`{0}` is not a valid frequency.")]
    InvalidFrequency(String),
    #[error("`{0}` is not a valid weekday.")]
    InvalidWeekday(String),
}
