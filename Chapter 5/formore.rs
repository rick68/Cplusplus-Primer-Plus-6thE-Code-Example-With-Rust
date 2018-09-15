// formore.rs -- more looping with for

use std::io;
use std::io::prelude::*;

const ARSIZE: usize = 16; // example of external declaration

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut factorials: [i64; ARSIZE] = <[i64; ARSIZE]>::default();
    factorials[0] = 1i64;
    factorials[1] = 1i64;
    for i in 2..ARSIZE {
        factorials[i] = i as i64 * factorials[i - 1];
    }
    for i in 0..ARSIZE {
        write!(stdout, "{}! = {}\n", i, factorials[i]);
    }

    Ok(())
}
