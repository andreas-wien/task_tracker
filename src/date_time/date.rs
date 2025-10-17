use std::fmt::Display;

#[derive(Clone)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>4}/{:0>2}/{:0>2}", self.year, self.month, self.day)
    }
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Date { day, month, year }
    }
}
