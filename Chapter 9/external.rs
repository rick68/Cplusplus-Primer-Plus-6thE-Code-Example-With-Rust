// external.rs -- external variable
// compile with support.rs

mod support;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    write!(stdout, "Global WARMING is {} degrees.\n", unsafe { support::WARMING });
    support::update(0.1);       // call function to change support::WARMING
    write!(stdout, "Global WARMING is {} degrees.\n", unsafe { support::WARMING });

    Ok(())
}
