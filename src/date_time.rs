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
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        const SECS_PER_MIN: u64 = 60;
        const SECS_PER_HOUR: u64 = 3600;
        const SECS_PER_DAY: u64 = 86400;

        let days_since_epoch = now / SECS_PER_DAY;
        let secs_today = now % SECS_PER_DAY;

        let hour = (secs_today / SECS_PER_HOUR) as u8;
        let minute = ((secs_today % SECS_PER_HOUR) / SECS_PER_MIN) as u8;
        let second = (secs_today % SECS_PER_MIN) as u8;

        let (year, month, day) = DateTime::days_to_ymd(days_since_epoch);

        DateTime { date: Date::new(year, month, day), time: Time::new(hour, minute, second) }
    }

    fn days_to_ymd(mut days: u64) -> (u16, u8, u8) {
        let mut year = 1970;
        loop {
            let leap = DateTime::is_leap_year(year);
            let year_days = if leap { 366 } else { 365 };
            if days < year_days {
                break;
            }
            days -= year_days;
            year += 1;
        }

        const MONTH_DAYS: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month = 1;
        for m in 0..12 {
            let mut d = MONTH_DAYS[m];
            if m == 1 && DateTime::is_leap_year(year) {
                d += 1;
            }
            if days < (d as u64) {
                return (year as u16, month, (days + 1) as u8);
            }
            days -= d as u64;
            month += 1;
        }

        (year as u16, 12, 31)
    }

    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }
}
