// hexoct1.rs -- shows hex and octal literals

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let chest: i32 = 42;        // decimal integer literal
    let waist: i32 = 0x42;      // hexadecimal integer literal
    let inseam: i32 = 042;      // octal integer literal

    stdout.write(b"Monsieur cuts a striking figure!\n")?;
    write!(stdout, "chest = {} (42 in decimal)\n", chest);
    write!(stdout, "waist = {} (0x42 in hex)\n", waist);
    write!(stdout, "inseam = {} (042 in octal)\n", inseam);

    Ok(())
}
