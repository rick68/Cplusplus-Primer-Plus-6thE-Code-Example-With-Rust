// hexoct2.rs -- display values in hex and octal

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let chest: i32 = 42;
    let waist: i32 = 42;
    let inseam: i32 = 42;

    stdout.write(b"Monsieur cuts a striking figure!\n")?;
    write!(stdout, "chest = {} (decimal for 42)\n", chest);
    write!(stdout, "waist = {:x} (hexadecimal for 42)\n", waist);
    write!(stdout, "inseam = {:o} (octal for 42)\n", inseam);

    Ok(())
}
