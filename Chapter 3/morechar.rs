// morechar.rs -- the char type and int type contrasted

use std::io;
use std::io::prelude::*;
use std::{char, u32};

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let ch: char = 'M';         // assign Unicode for M to ch
    let i: u32 = ch as u32;     // store same code in an i64
    write!(stdout, "The ASCII code for {} is {}\n", ch, i);

    stdout.write(b"Add one to the character code:\n")?;
    let ch: char = char::from_u32(ch as u32 + 1).unwrap();      // change character code in chB
    let i: u32 = ch as u32;                                     // save new character code in i
    write!(stdout, "The ASCII code for {} is {}\n", ch, i);

    // using the stdout.write_fmt() method to display a char
    stdout.write(b"Displaying char ch using stdout.write_fmt(format_args!(\"{}\", ch)): ")?;
    stdout.write_fmt(format_args!("{}", ch))?;

    // using the write!() to display a char constant
    write!(stdout, "{}", '!');

    stdout.write(b"\nDone\n")?;

    Ok(())
}
