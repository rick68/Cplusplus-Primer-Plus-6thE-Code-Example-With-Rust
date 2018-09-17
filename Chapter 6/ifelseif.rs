// ifelseif.rs -- using the if else if statement

use std::io;
use std::io::prelude::*;
use std::string::String;

const FAVE: i32 = 27;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut n: i32;
    let _: usize = stdout.write(b"Enter a number in the range 1-100 to find ")?;
    let _: usize = stdout.write(b"my favorite number: ")?;
    let _: () = stdout.flush()?;
    let mut input: String = String::new();
    loop {
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        n = input.trim().parse::<i32>().unwrap_or(i32::default());
        if n < FAVE {
            let _: usize = stdout.write(b"Too low -- guess again: ")?;
            let _: () = stdout.flush()?;
        } else if n > FAVE {
            let _: usize = stdout.write(b"Too high -- guess again: ")?;
            let _: () = stdout.flush()?;
        } else {
            write!(stdout, "{} is right!\n", FAVE);
        }

        if !(n != FAVE) {
            break;
        }
    }

    Ok(())
}
