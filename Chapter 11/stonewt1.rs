// stonewt1.rs -- Stonewt struct methods + conversion functions

use std::io;
use std::io::prelude::*;

use std::default::Default;
use std::ops::Drop;
use std::convert::Into;

const LBS_PER_STN: usize = 14; // pounds per stone

#[derive(Clone)]
pub struct Stonewt {
    stone: usize,  // whole stones
    pds_left: f64, // fractional pounds
    pounds: f64,   // entire weight in pounds
}

impl Default for Stonewt {
    // default constructor, wt = 0
    fn default() -> Self {
        Self {
            stone: 0,
            pds_left: 0.0,
            pounds: 0.0,
        }
    }
}

impl Drop for Stonewt {
    // destructor
    fn drop(&mut self) {}
}

macro_rules! Stonewt_new {
    () => {
        Stonewt::default()
    };
    ($lbs:expr) => {
        Stonewt::new($lbs)
    };
    ($stn:expr, $lbs:expr) => {
        Stonewt::new_with_stn($stn, $lbs)
    };
}

impl Stonewt {
    // construct Stonewt object from f64 value
    pub fn new(lbs: f64) -> Self {
        Self {
            stone: lbs as usize / LBS_PER_STN, // integer division
            pds_left: lbs % LBS_PER_STN as f64 + lbs - lbs.trunc(),
            pounds: lbs,
        }
    }

    // construct Stonewt object from stone, f64 value
    pub fn new_with_stn(stn: usize, lbs: f64) -> Self {
        Self {
            stone: stn,
            pds_left: lbs,
            pounds: (stn * LBS_PER_STN) as f64 + lbs,
        }
    }

    // show weight in stones
    pub fn show_stn(&self) {
        let mut stdout: io::Stdout = io::stdout();
        write!(stdout, "{} stone, {} pounds\n", self.stone, self.pds_left);
    }

    // show weight in pounds
    pub fn show_lbs(&self) {
        let mut stdout: io::Stdout = io::stdout();
        write!(stdout, "{} pounds\n", self.pounds);
    }
}

impl Into<usize> for Stonewt {
    fn into(self) -> usize {
        (self.pounds + 0.5) as usize
    }
}

impl Into<f64> for Stonewt {
    fn into(self) -> f64 {
        self.pounds
    }
}
