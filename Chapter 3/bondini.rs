// bondini.rs -- using escape sequences

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    stdout.write(b"Operation \"HyperHype\" is now activated!\n")?;
    stdout.write(b"Enter your agent code: ")?;
    stdout.flush()?;
    let code: i64;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    code = input.trim().parse::<i64>().unwrap_or(i64::default());
    write!(stdout, "You entered {}...\n", code);
    stdout.write(b"Code verified! proceed with Plan Z3!\n")?;

    Ok(())
}
