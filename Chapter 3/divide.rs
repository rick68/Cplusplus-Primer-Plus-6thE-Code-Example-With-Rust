// divide.rs -- interger and floating-point division

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    write!(stdout, "Integer division: 9/5 = {}\n", 9 / 5);
    stdout.write(b"Floating-point division: 9.0/5.0 = ")?;
    write!(stdout, "{:.*}\n", 6, 9.0 / 5.0);
    stdout.write(b"double constants: 1.0e7f64/9.0f64 = ")?;
    write!(stdout, "{:.*}\n", 6, 1.0e7f64 / 9.0f64);
    stdout.write(b"float constants: 1.0e7f32/9.0f32 = ")?;
    write!(stdout, "{:.*}\n", 6, 1.0e7f32 / 9.0f32);

    Ok(())
}
