// compstr2.rs -- comparing strings using arrays

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut word: String = String::from("?ate");

    for ch in ('a' as u8).. {
        if word == "mate" {
            break;
        }
        write!(stdout, "{}\n", word);
        unsafe { word.as_mut_vec()[0] = ch };
    }

    write!(stdout, "After loop ends, word is {}\n", word);

    Ok(())
}
