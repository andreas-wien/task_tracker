mod date;
mod time;

use std::{ fmt::Display, str::FromStr, time::{ SystemTime, UNIX_EPOCH } };

use crate::date_time::{ date::Date, time::Time };

#[derive(Clone)]
pub struct DateTime {
    date: Date,
    time: Time,
}

pub struct DateTimeParseError;

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

impl Default for DateTime {
    fn default() -> Self {
        DateTime::now()
    }
}

impl FromStr for DateTime {
    type Err = DateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_time: Vec<&str> = s.split_whitespace().collect();
        let date: Vec<&str> = date_time[0].split("/").collect();
        let year = date[0].parse().unwrap_or_default();
        let month = date[1].parse().unwrap_or_default();
        let day = date[2].parse().unwrap_or_default();
        let time: Vec<&str> = date_time[1].split(":").collect();
        let hour = time[0].parse().unwrap_or_default();
        let minute = time[1].parse().unwrap_or_default();
        let second = time[2].parse().unwrap_or_default();

        Ok(DateTime { date: Date::new(year, month, day), time: Time::new(hour, minute, second) })
    }
}

impl DateTime {
    pub fn now() -> DateTime {
        let ms_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let year = 0;
        let month = 0;
        let day = 0;
        let hour = 0;
        let minute = 0;
        let second = 0;

        DateTime { date: Date::new(year, month, day), time: Time::new(hour, minute, second) }
    }
}
