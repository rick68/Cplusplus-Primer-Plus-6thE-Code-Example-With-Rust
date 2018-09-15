// block.rs -- use a block statement

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    stdout.write(b"The Amazing Accounto will sum and average ")?;
    stdout.write(b"five numbers for you.\n")?;
    stdout.write(b"Please enter five values:\n")?;
    let mut number: f64 = 0.0;
    let mut sum: f64 = 0.0;
    let mut input: String = String::new();
    for i in 1..=5 {                            // block starts here
        write!(stdout, "Value {}: ", i);
        stdout.flush()?;
        loop {
            input.clear();
            let _: usize = stdin.read_line(&mut input)?;
            let trimed: &str = input.trim();
            if trimed.len() == 0 {
                continue;
            }
            number = trimed.parse::<f64>().unwrap_or(f64::default());
            break;
        }
        sum += number;
    }                                           // block ends here
    stdout.write(b"Five exquisite choices indeed! ")?;
    write!(stdout, "They sum to {}\n", sum);
    write!(stdout, "and average to {}.\n", sum / 5.0);
    stdout.write(b"The Amazing Accounto bids you adieu!\n")?;

    Ok(())
}
