// ourfunc.rs -- defining your own function

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    simon(3);           // call the simon() function

    stdout.write(b"Pick an integer: ")?;
    stdout.flush()?;
    let count: i32;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    count = input.trim().parse::<i32>().unwrap_or(i32::default());
    simon(count);       // call it again
    stdout.write(b"Down!\n")?;

    Ok(())
}

fn simon(n: i32)  // define the simon() function
{
    println!("Simon says touch your toes {} times.", n);
}                 // functions don't need return statements
