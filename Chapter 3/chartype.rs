// chartype.rs -- the char type

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let ch: char; // declare a char variable
    stdout.write(b"Enter a character: \n")?;
    let mut input: String = String::new();
    loop {
        if let Ok(n) = stdin.read_line(&mut input) {
            if !(n == 0 || input.trim().len() == 0) {
                ch = input.trim().chars().next().unwrap();
                break;
            }
        }
        input.clear();
    }

    stdout.write(b"Hola! ")?;
    write!(stdout, "Thank you for the {} character.\n", ch);

    Ok(())
}
