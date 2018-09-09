// sqrt.rs -- using the sqrt() function

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let area: f64;
    stdout.write(b"Enter the floor area, in square feet, of your home: ")?;
    stdout.flush()?;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    area = input.trim().parse::<f64>().unwrap_or(f64::default());
    let side: f64;
    side = area.sqrt();
    write!(
        stdout,
        "That's the equivalent of a square {} feet to the side.\n",
        side
    )?;
    stdout.write(b"How fascinating!\n")?;

    Ok(())
}
