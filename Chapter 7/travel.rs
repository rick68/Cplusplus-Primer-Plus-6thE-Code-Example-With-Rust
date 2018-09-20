// travel.rs -- using structures with functions

use std::io;
use std::io::prelude::*;
use std::clone::Clone;
use std::marker::Copy;

struct TravelTime {
    hours: i32,
    mins: i32,
}

impl Clone for TravelTime {
    fn clone(&self) -> TravelTime {
        *self
    }
}

impl Copy for TravelTime {}

const MINS_PER_HR: i32 = 60;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let day1: TravelTime = TravelTime {
        hours: 5,
        mins: 45,
    };
    let day2: TravelTime = TravelTime {
        hours: 4,
        mins: 55,
    };

    let trap: TravelTime = sum(day1, day2);
    stdout.write(b"Two-day total: ")?;
    show_time(trap);

    Ok(())
}

fn sum(t1: TravelTime, t2: TravelTime) -> TravelTime {
    let total: TravelTime = TravelTime {
        mins: (t1.mins + t2.mins) % MINS_PER_HR,
        hours: t1.hours + t2.hours + (t1.mins + t2.mins) / MINS_PER_HR,
    };
    total
}

fn show_time(t: TravelTime) {
    let mut stdout: io::Stdout = io::stdout();
    write!(stdout, "{} hours, {} minutes\n", t.hours, t.mins);
}
