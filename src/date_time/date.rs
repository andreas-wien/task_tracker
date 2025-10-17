use std::fmt::Display;

pub struct Date {
    day: u8,
    month: u8,
    year: i32,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.year, self.month, self.day)
    }
}
