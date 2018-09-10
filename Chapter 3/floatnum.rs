// floatnum.rs -- floating-point types

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let tub: f32 = 10.0 / 3.0;  // good to about 6 places
    let mint: f64 = 10.0 / 3.0; // good to about 15 places
    const MILLION: f32 = 1.0e6;

    write!(stdout, "tub = {:.*}", 6, tub)?;
    write!(stdout, ", a million tubs = {:.*}", 6, MILLION * tub);
    stdout.write(b",\nand ten million tubs = ")?;
    write!(stdout, "{:.*}\n", 6, 10.0 * MILLION * tub)?;

    write!(stdout, "mint = {:.*} and a million mints = ", 6, mint);
    write!(stdout, "{:.*}\n", 6, MILLION as f64 * mint);

    Ok(())
}
