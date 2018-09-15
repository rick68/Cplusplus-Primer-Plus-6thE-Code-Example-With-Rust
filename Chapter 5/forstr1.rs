// forstr1.rs -- using for with a string

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let _: usize = stdout.write(b"Enter a word: ")?;
    let _: () = stdout.flush()?;
    let mut word: String;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    word = input.trim().to_string();

    // display letters in reverse order
    for i in (0..word.len()).rev() {
        let c: u8 = unsafe { word.as_mut_vec()[i] };
        write!(stdout, "{}\n", c as char);
    }
    write!(stdout, "\nBye.\n");

    Ok(())
}
