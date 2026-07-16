use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Self::wrap(hours, minutes);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Self::wrap(self.hours, self.minutes + minutes);
        Self { hours, minutes }
    }

    fn wrap(hours: i32, minutes: i32) -> (i32, i32) {
        // remainders
        let hours_from_minutes = minutes / 60;
        let minutes = minutes % 60;

        let total_hours = hours + hours_from_minutes;
        let hours = total_hours % 24;

        // wrap negative minutes
        let (hours, minutes) = if minutes < 0 {
            (hours - 1, 60 + minutes)
        } else {
            (hours, minutes)
        };

        // wrap negative hours
        if hours < 0 {
            (24 + hours, minutes)
        } else {
            (hours, minutes)
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
