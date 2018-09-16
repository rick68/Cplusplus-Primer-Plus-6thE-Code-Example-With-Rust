// num_test.rs -- use numeric test in for loop

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let _: usize = stdout.write(b"Enter the strating countdown value: ")?;
    let _: () = stdout.flush()?;
    let limit: i32;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    limit = input.trim().parse::<i32>().unwrap_or(i32::default());
    for i in (0..limit).rev() {
        if i == 0 { break; }
        write!(stdout, "i = {}\n", i);
    }

    Ok(())
}
