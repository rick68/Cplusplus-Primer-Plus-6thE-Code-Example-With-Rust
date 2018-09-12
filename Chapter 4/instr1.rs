// insta1.rs -- reading more than one string

use std::io;
use std::io::prelude::*;
use std::str;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    const ARSIZE: usize = 20;
    let mut name: [u8; ARSIZE] = [0; ARSIZE];
    let mut dessert: [u8; ARSIZE] = [0; ARSIZE];

    stdout.write(b"Enter your name:\n")?;
    let n = stdin.read(&mut name)?;
    name[n - 1] = 0; // remove newline
    stdout.write(b"Enter your favorite dessert:\n")?;
    let n = stdin.read(&mut dessert)?;
    dessert[n - 1] = 0; // remove newline
    write!(stdout, "I have some delicious {}", str::from_utf8(&dessert).unwrap());
    write!(stdout, " for you, {}.\n", str::from_utf8(&name).unwrap());

    Ok(())
}
