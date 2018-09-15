// bigstep.rs -- count as directed

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    stdout.write(b"Enter an integer: ")?;
    stdout.flush()?;
    let by: i32;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    by = input.trim().parse::<i32>().unwrap_or(i32::default());
    write!(stdout, "Counting by {}s:\n", by);
    for i in (0..100).step_by(by as usize) {
        write!(stdout, "{}\n", i);
    }

    Ok(())
}
