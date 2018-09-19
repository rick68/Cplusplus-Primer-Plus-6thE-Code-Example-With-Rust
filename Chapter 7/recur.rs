// recur.rs -- using recursion

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    countdown(4); // call the recursive function

    Ok(())
}

fn countdown(n: i32) {
    let mut stdout: io::Stdout = io::stdout();
    write!(stdout, "Counting down ... {}\n", n);
    if n > 0 {
        countdown(n - 1); // function calls itself
    }
    write!(stdout, "{}:Kaboom!\n", n);
}
