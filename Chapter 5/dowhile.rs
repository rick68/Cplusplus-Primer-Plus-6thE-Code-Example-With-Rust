// dowhile.rs -- exit-condition loop

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();
    let mut n: i32 = 0;

    write!(stdout, "Enter numbers in the range 1-10 to find ");
    write!(stdout, "my favorite number\n");

    let mut input: String = String::new();
    loop {
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        n = input.trim().parse::<i32>().unwrap_or(i32::default());
        if !(n != 7) {
            break;
        }
    }
    stdout.write(b"Yes, 7 is my favorite.\n")?;

    Ok(())
}
