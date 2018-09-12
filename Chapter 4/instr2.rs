// insta1.rs -- reading a line

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut name: String = String::new();
    let mut dessert: String = String::new();

    stdout.write(b"Enter your name:\n")?;
    stdin.read_line(&mut name)?;
    stdout.write(b"Enter your favorite dessert:\n")?;
    stdin.read_line(&mut dessert)?;
    write!(stdout, "I have some delicious {}", dessert.trim());
    write!(stdout, " for you, {}.\n", name.trim());

    Ok(())
}
