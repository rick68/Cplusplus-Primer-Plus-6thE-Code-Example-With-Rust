// strgback.rs -- a function that returns a pointer to char

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::boxed::Box;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let times: usize;
    let mut ch: [u8; 2] = <[u8; 2]>::default();

    let _: usize = stdout.write(b"Enter a character: ")?;
    stdout.flush()?;
    let _: usize = stdin.read(&mut ch)?;
    let _: usize = stdout.write(b"Enter an integer: ")?;
    stdout.flush()?;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    times = input.trim().parse::<usize>().unwrap_or(usize::default());
    let mut ps: Box<Vec<u8>> = buildstr(ch[0], times);
    write!(stdout, "{}\n", String::from_utf8(*ps).unwrap());
    ps = buildstr('+' as u8, 20);
    write!(stdout, "{0} -DONE- {0}\n", String::from_utf8(*ps).unwrap());

    Ok(())
}

// builds, string made of n c characters
fn buildstr(c: u8, n: usize) -> Box<Vec<u8>> {
    let mut pstr: Box<Vec<u8>> = Box::new(vec![0; n+1]);

    pstr[n] = 0;        // terminate string
    for i in 0..n {
        pstr[i] = c;    // fill rest of string
    }
    pstr
}
