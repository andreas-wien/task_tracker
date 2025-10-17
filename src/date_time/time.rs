use std::fmt::Display;

#[derive(Clone)]
pub struct Time {
    second: u8,
    minute: u8,
    hour: u8,
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
    }
}


impl Time {
    pub fn new(hour: u8, minute: u8, second: u8) -> Self {
        Time { second, minute, hour }
    }
}
