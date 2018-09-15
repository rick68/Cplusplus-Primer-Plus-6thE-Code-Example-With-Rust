// forloop.rs -- introducing the for loop

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    for i in 0..5 {
        let _: usize = stdout.write(b"Rust knows loops.\n")?;
    }
    let _: usize = stdout.write(b"Rust when to stop.\n")?;

    Ok(())
}
