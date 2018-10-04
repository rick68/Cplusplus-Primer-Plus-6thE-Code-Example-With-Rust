// mytime2.rs -- Time struct

use std::io;
use std::io::prelude::*;

use std::default::Default;
use std::ops::{Add, Mul, Sub};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
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
        write!(stdout, "{}", self);
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

// operator* overloading
impl Mul<f64> for Time {
    type Output = Time;

    fn mul(self, rhs: f64) -> Time {
        let totalminutes: f64 = self.hours as f64* rhs * 60.0 + self.minutes as f64 * rhs;
        Time {
            hours: (totalminutes / 60.0) as i32,
            minutes: (totalminutes % 60.0) as i32,
        }
    }
}
impl Mul<Time> for f64 {
    type Output = Time;

    fn mul(self, rhs: Time) -> Time {
        let totalminutes: f64 = rhs.hours as f64* self * 60.0 + rhs.minutes as f64 * self;
        Time {
            hours: (totalminutes / 60.0) as i32,
            minutes: (totalminutes % 60.0) as i32,
        }
    }
}

// operator- overloading
impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        let tot1: i32 = self.minutes + 60 * self.hours;
        let tot2: i32 = other.minutes + 60 * other.hours;
        let diff: i32 = tot1 - tot2;
        Time {
            hours: diff / 60,
            minutes: diff % 60,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} hours, {} minutes", self.hours, self.minutes)
    }
}
