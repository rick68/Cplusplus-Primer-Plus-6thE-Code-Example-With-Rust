// convert.rs -- converts stone to pounds

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let stone: i32;
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    stdout.write(b"Enter the weight in stone: ")?;
    stdout.flush()?;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    stone = input.trim().parse::<i32>().unwrap_or(i32::default());
    let pounds: i32 = stonetolb(stone);
    write!(stdout, "{} stone = ", stone);
    write!(stdout, "{} pounds.", pounds);

    Ok(())
}

fn stonetolb(sts: i32) -> i32 {
    14 * sts
}
