mod date;
mod time;

use std::fmt::Display;

use crate::date_time::{ date::Date, time::Time };

pub struct DateTime {
    date: Date,
    time: Time,
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}
