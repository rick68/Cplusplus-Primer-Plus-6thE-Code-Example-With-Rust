// usetime2.rs -- using the third draft of the Time struct
// compile usetime2.rs and mytime2.rs together

#[macro_use]
mod mytime2;
use mytime2::*;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let weeding: Time = Time_new!(4, 35);
    let waxing: Time = Time_new!(2, 47);
    let total: Time;
    let diff: Time;
    let adjusted: Time;

    let _: usize = stdout.write(b"weeding time = ")?;
    weeding.show();
    let _: usize = stdout.write(b"\n")?;

    let _: usize = stdout.write(b"waxing time = ")?;
    waxing.show();
    let _: usize = stdout.write(b"\n")?;

    let _: usize = stdout.write(b"total work time = ")?;
    total = weeding + waxing; // use std::ops::Add::add()
    total.show();
    let _: usize = stdout.write(b"\n")?;

    diff = weeding - waxing; // use std::ops::Sub::sub()
    let _: usize = stdout.write(b"weeding time - waxing time = ")?;
    diff.show();
    let _: usize = stdout.write(b"\n")?;

    adjusted = total * 1.5; // use std::ops::Mul::mul()
    let _: usize = stdout.write(b"adjusted work time = ")?;
    adjusted.show();
    let _: usize = stdout.write(b"\n")?;

    Ok(())
}
