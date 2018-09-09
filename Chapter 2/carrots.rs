// carrots.rs -- food processing program
// uses and displays a variable

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut carrots: i32; // declare an integer variable

    carrots = 25; // assign a value to the variable
    stdout.write(b"I have ")?;
    io::stdout().write(carrots.to_string().as_bytes())?; // display the value of the variable
    io::stdout().write(b" carrots.")?;
    io::stdout().write(b"\n")?;
    carrots = carrots - 1; // modify the variable
    write!(stdout, "Crunch, crunch, Now I have {} cattorts.", carrots)?;

    Ok(())
}
