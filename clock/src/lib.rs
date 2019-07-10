use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_mins = hours * 60 + minutes;
        let mut hours = f64::floor(total_mins as f64 / 60_f64) as i32 % 24;
        let mut minutes = total_mins % 60;

        if hours < 0 {
            hours = hours + 24;
        }
        if minutes < 0 {
            minutes = minutes + 60;
        }

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        Clock::new(self.hours, new_minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}
