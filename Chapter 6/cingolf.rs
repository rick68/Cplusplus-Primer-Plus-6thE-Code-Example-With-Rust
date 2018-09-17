// cingolf.rs -- non-numeric input skipped

use std::io;
use std::io::prelude::*;
use std::string::String;

const MAX: usize = 5;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    // get data
    let mut golf: [i32; MAX] = <[i32; MAX]>::default();
    let _: usize = stdout.write(b"Please enter your glof scores.\n")?;
    write!(stdout, "You must enter {} rounds.\n", MAX);
    let mut input: String = String::new();
    for i in 0..MAX {
        write!(stdout, "round #{}: ", i + 1);
        let _: () = stdout.flush()?;
        loop {
            input.clear();
            let _: usize = stdin.read_line(&mut input)?;
            match input.trim().parse::<i32>() {
                Ok(val) => {
                    golf[i] = val;
                    break;
                },
                Err(_) => {
                    let _: usize = stdout.write(b"Please enter a number: ")?;
                    let _: () = stdout.flush()?;
                    continue;
                },
            }
        }
    }
    // calculate average
    let mut total: f64 = 0.0;
    for i in 0..MAX {
        total += golf[i] as f64;
    }
    // report results
    write!(stdout, "{} = average score {} rounds\n", total / MAX as f64, MAX);

    Ok(())
}
