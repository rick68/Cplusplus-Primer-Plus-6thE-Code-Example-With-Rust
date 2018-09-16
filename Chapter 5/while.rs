// while.rs -- introducing the while loop

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
    
const ARSIZE: usize = 20;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut name: [u8; ARSIZE] = <[u8; ARSIZE]>::default();
    let _: usize = stdout.write(b"Your first name, please: ")?;
    let _: () = stdout.flush()?;
    let n: usize = stdin.read(&mut name)?;
    name[n - 1] = 0; // remove newline
    let _: usize = stdout.write(b"Here is your name, verticalized and ASCIIized:\n")?;
    let mut i: usize = 0;               // start at beginning of string
    while name[i] != '\0' as u8 {       // process to end of string
        write!(stdout, "name[i]: {}\n", name[i]);
        i += 1;                         // don't forget this step
    }

    Ok(())
}
