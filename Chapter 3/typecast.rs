// typecast.rs -- forcing tyBpe changes

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let auks: i32;
    let bats: i32;
    let coots: i32;

    // the following statement adds the values as f64,
    // then converts the result to i32
    auks = (19.99 + 11.99) as i32;

    // these statements add values as i32
    bats = 19.99 as i32 + 11.99 as i32;
    coots = 19.99 as i32 + 11.99 as i32;
    write!(stdout, "auks = {}, bats = {}", auks, bats);
    write!(stdout, ", coots = {}\n", coots);

    let ch: char = 'Z';
    write!(stdout, "The code for {} is ", ch);  // print as char
    write!(stdout, "{}\n", ch as i32);          // print as i32
    stdout.write(b"Yes, the code is ")?;
    write!(stdout, "{}\n", ch as i32);

    Ok(())
}
