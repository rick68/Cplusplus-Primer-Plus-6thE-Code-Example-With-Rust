// calling.rs -- defining, prototyping, and calling a function

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    stdout.write(b"main() will call the simple() function:\n")?;
    simple();   // function call
    stdout.write(b"main() is finished with the simple() function.\n")?;

    Ok(())
}

// function definition
fn simple() {
    let mut stdout: io::Stdout = io::stdout();
    stdout.write(b"I'm but a simple function.\n").unwrap();
}
