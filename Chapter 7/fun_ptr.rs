// fun_ptr.rs -- pointers to functions

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let code: i32;

    let _: usize = stdout.write(b"How many lines of code do you need? ")?;
    stdout.flush()?;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    code = input.trim().parse::<i32>().unwrap_or(i32::default());
    let _: usize = stdout.write(b"Here's Betsy's estimate:\n")?;
    estimate(code, betsy);
    let _: usize = stdout.write(b"Here's Pam's estimate:\n")?;
    estimate(code, pam);

    Ok(())
}

fn betsy(lns: i32) -> f64 {
    0.05 * lns as f64
}

fn pam(lns: i32) -> f64 {
    0.03 * lns as f64 + 0.0004 * lns as f64 * lns as f64
}

fn estimate(lines: i32, pf: fn(i32) -> f64) {
    let mut stdout: io::Stdout = io::stdout();
    write!(stdout, "{} lines will take ", lines);
    write!(stdout, "{} hour(s)\n", pf(lines));
}
