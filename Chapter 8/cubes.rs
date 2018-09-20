// cubes.rs -- regular and reference arguments

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut x: f64 = 3.0;

    write!(stdout, "{}", cube(x));
    write!(stdout, " = cube of {}\n", x);
    write!(stdout, "{}", refcube(&mut x));
    write!(stdout, " = cube of {}\n", x);

    Ok(())
}

fn cube(mut a: f64) -> f64 {
    a *= a * a;
    a
}

fn refcube(ra: &mut f64) -> f64 {
    *ra *= *ra * *ra;
    *ra
}
