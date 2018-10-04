// usetime0.rs -- using the first draft of the Time struct
// compile usetime0.rs and mytime0.rs together

#[macro_use]
mod mytime0;
use mytime0::*;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let planning: Time = Time_new!();
    let coding: Time = Time_new!(2, 40);
    let fixing: Time = Time_new!(5, 55);
    let total: Time;

    let _: usize = stdout.write(b"planning time = ")?;
    planning.show();
    let _: usize = stdout.write(b"\n")?;

    let _: usize = stdout.write(b"coding time = ")?;
    coding.show();
    let _: usize = stdout.write(b"\n")?;

    let _: usize = stdout.write(b"fixing time = ")?;
    fixing.show();
    let _: usize = stdout.write(b"\n")?;

    total = coding.sum(&fixing);
    let _: usize = stdout.write(b"coding.sum(&fixing) = ")?;
    total.show();
    let _: usize = stdout.write(b"\n")?;

    Ok(())
}
