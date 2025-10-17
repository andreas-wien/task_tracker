use std::fmt::Display;

pub struct Time {
    second: u8,
    minute: u8,
    hour: u8,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.hour, self.minute, self.second)
    }
}
