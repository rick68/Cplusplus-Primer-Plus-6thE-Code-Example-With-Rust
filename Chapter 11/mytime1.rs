// mytime1.rs -- Time struct

use std::io;
use std::io::prelude::*;

use std::default::Default;
use std::ops::Add;

#[derive(Clone)]
pub struct Time {
    hours: i32,
    minutes: i32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
        }
    }
}

macro_rules! Time_new {
    () => {
        Time::default()
    };
    ($h:expr) => {
        Time::new($h, 0)
    };
    ($h:expr, $m:expr) => {
        Time::new($h, $m)
    };
}

impl Time {
    pub fn new(h: i32, m: i32) -> Time {
        Time {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_min(&mut self, m: i32) {
        self.minutes += m;
        self.hours += self.minutes / 60;
        self.minutes %= 60;
    }

    pub fn add_hr(&mut self, h: i32) {
        self.hours += h;
    }

    pub fn reset(&mut self, h: i32, m: i32) {
        self.hours = h;
        self.minutes = m;
    }

    pub fn show(&self) {
        let mut stdout: io::Stdout = io::stdout();
        write!(stdout, "{} hours, {} minutes", self.hours, self.minutes);
    }
}

// operator+ overloading
impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        let mut sum: Time = self;
        sum.add_hr(other.hours);
        sum.add_min(other.minutes);
        sum
    }
}
