// getinfo.rs -- input and output

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut carrots: i32;
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    stdout.write(b"How many carrots do you have?\n")?;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?; // rust input
    carrots = input.trim().parse::<i32>().unwrap_or(i32::default());
    stdout.write(b"Here are two more. \n")?;
    carrots = carrots + 2;
    // the next line concatenates output
    write!(stdout, "Now you have {} carrots.", carrots);

    Ok(())
}
