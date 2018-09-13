// numstr.rs -- flowwing number input with line input

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    stdout.write(b"What year was your house built?\n")?;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    let year: i32 = input.trim().parse::<i32>().unwrap_or(i32::default());
    stdout.write(b"What is its street address?\n")?;
    let mut address: String = String::new();
    stdin.read_line(&mut address)?;
    write!(stdout, "Year built: {}\n", year);
    write!(stdout, "Address: {}\n", address);
    stdout.write(b"Done!\n")?;

    Ok(())
}
