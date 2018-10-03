// stone1.rs -- user-defined conversions functions
// compile with stonewt1.rs

#[macro_use]
mod stonewt1;
use stonewt1::*;

use std::io;
use std::io::prelude::*;
use std::convert::Into;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let poppins: Stonewt = Stonewt_new!(9, 2.8); // 9 stone, 2.8 pounds
    let p_wt: f64 = poppins.clone().into(); // inmplicit conversion

    let _: usize = stdout.write(b"Convert to f64 => ")?;
    write!(stdout, "Popins: {} pounds.\n", p_wt);
    let _: usize = stdout.write(b"Convert to usize => ")?;
    write!(stdout, "Popins: {} pounds.\n", Into::<usize>::into(poppins));

    Ok(())
}
