// usetime1.rs -- using the second draft of the Time struct
// compile usetime1.rs and mytime1.rs together

#[macro_use]
mod mytime1;
use mytime1::*;

use std::io;
use std::io::prelude::*;
use std::ops::Add;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let planning: Time = Time_new!();
    let coding: Time = Time_new!(2, 40);
    let fixing: Time = Time_new!(5, 55);
    let mut total: Time;

    let _: usize = stdout.write(b"planning time = ")?;
    planning.show();
    let _: usize = stdout.write(b"\n")?;

    let _: usize = stdout.write(b"coding time = ")?;
    coding.show();
    let _: usize = stdout.write(b"\n")?;

    let _: usize = stdout.write(b"fixing time = ")?;
    fixing.show();
    let _: usize = stdout.write(b"\n")?;

    total = coding + fixing;
    let _: usize = stdout.write(b"coding + fixing = ")?;
    total.show();
    let _: usize = stdout.write(b"\n")?;

    let morefixing: Time = Time_new!(3, 28);
    let _: usize = stdout.write(b"more fixing = ")?;
    morefixing.show();
    let _: usize = stdout.write(b"\n")?;
    total = Add::add(morefixing, total);
    // function notation
    let _: usize = stdout.write(b"Add::add(morefixing, total) = ")?;
    total.show();
    let _: usize = stdout.write(b"\n")?;

    Ok(())
}
