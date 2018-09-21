// inline.rs -- using an inline function

use std::io;
use std::io::prelude::*;

// an inline function definition
#[inline]
fn square(x: f64) -> f64 { x * x }

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let a: f64;
    let b: f64;
    let mut c: f64 = 13.0;

    a = square(5.0);
    b = square(4.5 + 7.5);      // can pass expressions

    write!(stdout, "a = {}, b = {}\n", a, b);
    write!(stdout, "c = {}", c);
    write!(stdout, ", c squared = {}\n", square(c));
    c += 1.0;
    write!(stdout, "Now c = {}\n", c);

    Ok(())
}
