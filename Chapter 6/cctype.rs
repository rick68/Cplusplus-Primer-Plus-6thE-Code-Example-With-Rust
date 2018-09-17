// cctype.rs -- use char

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let _: usize = stdout.write(b"Enter text for analysis, and type #")?;
    let _: usize = stdout.write(b" to terminate input.\n")?;

    let mut ch: [u8; 1] = <[u8; 1]>::default();
    let mut whitespace: usize = 0;
    let mut digits: usize = 0;
    let mut chars: usize = 0;
    let mut punct: usize = 0;
    let mut others: usize = 0;

    let _: () = stdin.read_exact(&mut ch)?; // get first character
    while ch[0] != '@' as u8 {
        // test for sentinel
        if ch[0].is_ascii_alphabetic() {        // is it an alphabetic character?
            chars += 1;
        } else if ch[0].is_ascii_whitespace() { // is it a whitespace character?
            whitespace += 1;
        } else if ch[0].is_ascii_digit() {      // is it a digit?
            digits += 1;
        } else if ch[0].is_ascii_punctuation() { // is it punctuation?
            punct += 1;
        } else {
            others += 1;
        }
        loop {
            let _: () = stdin.read_exact(&mut ch)?; // get next character
            if ch[0] == '\n' as u8 {
                continue;
            }
            break;
        }
    }
    write!(
        stdout,
        "{} letters, {} whitespace, {} digits, {} punctuations, {} others.\n",
        chars, whitespace, digits, punct, others
    );

    Ok(())
}
