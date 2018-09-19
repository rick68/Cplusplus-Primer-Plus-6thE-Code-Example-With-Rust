// lotto.rs -- probability of winning

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut total: f64;
    let mut choices: f64;

    let _: usize = stdout.write(b"Enter the total number of choices on the game card and\n")?;
    let _: usize = stdout.write(b"the number of picks allowed:\n")?;
    let mut input: String = String::new();

    loop {
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        total = match input.trim().parse::<f64>() {
            Ok(val) => val,
            _ => { break; },
        };
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        choices = match input.trim().parse::<f64>() {
            Ok(val) => val,
            _ => { break; },
        };
        if !(choices <= total) { break; }
        let _: usize = stdout.write(b"You have one chance in ")?;
        write!(stdout, "{}", probability(total as usize, choices as usize)); // compute the odds
        let _: usize = stdout.write(b" of winning.\n")?;
        let _: usize = stdout.write(b"Next two numbers (q to quit): ")?;
        stdout.flush()?;
    }
    let _: usize = stdout.write(b"bye\n")?;

    Ok(())
}

// the follwing function calculates the probability of pincking picks
// numbers correctly from numbers choices
fn probability(numbers: usize, picks: usize) -> f64 {
    let mut result: f64 = 1.0; // here come some local variables

    for (n, p) in (0..=numbers).rev().zip((1..=picks).rev()) {
        result = result * n as f64 / p as f64;
    }
    result
}
