// modulus.rs -- uses % operator to convert lbs to stone

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    const LBS_PER_STN: i32 = 14;
    let lbs: i32;

    stdout.write(b"Enter your weight in pounds: ")?;
    stdout.flush()?;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    lbs = input.trim().parse::<i32>().unwrap_or(i32::default());

    let stone: i32 = lbs / LBS_PER_STN;         // whole stone
    let pounds: i32 = lbs % LBS_PER_STN;        // remainder in pounds
    write!(stdout, "{} pounds are {} stone, {} pound(s).\n", lbs, stone, pounds);


    Ok(())
}
