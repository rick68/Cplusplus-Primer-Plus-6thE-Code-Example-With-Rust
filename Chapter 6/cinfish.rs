// cinfish.rs -- non-numeric input terminates loop

use std::io;
use std::io::prelude::*;
use std::string::String;

const MAX: usize = 5;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut fish: [f64; MAX] = <[f64; MAX]>::default();
    let _: usize = stdout.write(b"Please enter the weights of your fish.\n")?;
    write!(
        stdout,
        "You may enter up to {} fish <q to terminate.>\n",
        MAX
    );
    let _: usize = stdout.write(b"fish #1: ")?;
    let _: () = stdout.flush()?;
    let mut i: usize = 0;
    let mut input: String = String::new();
    'while_loop: while i < MAX {
        loop {
            input.clear();
            let n: usize = stdin.read_line(&mut input)?;
            if n == 0 {
                break 'while_loop;
            } // EOF
            else if input == "\n" {
                continue;
            } else {
                break;
            }
        }
        fish[i] = input.trim().parse::<f64>().unwrap_or(f64::default());
        i += 1;
        if i < MAX {
            write!(stdout, "fish #{}: ", i + 1);
            let _: () = stdout.flush()?;
        }
    }
    // calculate average
    let mut total: f64 = 0.0;
    for j in 0..i {
        total += fish[j];
    }
    // report results
    if i == 0 {
        let _: usize = stdout.write(b"No fish\n")?;
    } else {
        write!(
            stdout,
            "{} = average weight of {} fish\n",
            total / i as f64,
            i
        );
    }
    let _: usize = stdout.write(b"Done.\n")?;

    Ok(())
}
