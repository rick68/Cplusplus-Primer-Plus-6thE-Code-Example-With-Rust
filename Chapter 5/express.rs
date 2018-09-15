// express.rs -- values of expressions

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut x: i32 = 0;

    let _: usize = stdout.write(b"The expression x = 100 has the value ")?;
    x = 100;
    write!(stdout, "{}\n", x);
    write!(stdout, "Now x = {}\n", x);
    let _: usize = stdout.write(b"The expression x < 3 has the value ")?;
    write!(stdout, "{}\n", x < 3);
    let _: usize = stdout.write(b"The expression x > 3 has the value ")?;
    write!(stdout, "{}\n", x > 3);

    Ok(())
}
