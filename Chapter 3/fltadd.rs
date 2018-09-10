// fltadd.rs -- precision problems with float

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let a: f32 = 2.34E+22;
    let b: f32 = a + 1.0;

    write!(stdout, "a' = {:e}\n", a);
    write!(stdout, "b - a = {}\n", b - a);

    Ok(())
}
