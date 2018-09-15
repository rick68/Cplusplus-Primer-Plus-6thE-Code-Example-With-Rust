// strtype4.rs -- line input

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::str;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut charr: [u8; 20] = <[u8; 20]>::default();
    let mut strr: String = String::new();

    write!(stdout, "Length of string in charr before input: {}\n", str::from_utf8(&charr).unwrap().len());
    write!(stdout, "Length of string in strr before input: {}\n", strr.len());
    stdout.write(b"Enter a line of text:\n")?;
    let n: usize = stdin.read(&mut charr)?;
    charr[n - 1] = 0; // remove newline
    write!(stdout, "You entered: {}\n", str::from_utf8(&charr).unwrap());
    stdout.write(b"Enter another line of text:\n")?;
    let _: usize = stdin.read_line(&mut strr)?;
    write!(stdout, "Length of string in charr before input: {}\n", str::from_utf8(&charr).unwrap().len());
    write!(stdout, "Length of string in strr before input: {}\n", strr.trim().len());

    Ok(())
}
