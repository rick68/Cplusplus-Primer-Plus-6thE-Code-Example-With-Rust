// compstr1.rs -- comparing strings using arrays

use std::io;
use std::io::prelude::*;
use std::ptr::copy_nonoverlapping;
use std::str;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut word: [u8; 4] = <[u8; 4]>::default();
    unsafe { copy_nonoverlapping("?ate".as_ptr(), word.as_mut_ptr(), 4) };

    for ch in ('a' as u8).. {
        {
            let s: &str = str::from_utf8(&word).unwrap();
            if s == "mate" {
                break;
            }
            write!(stdout, "{}\n", s);
        }
        word[0] = ch;
    }

    write!(
        stdout,
        "After loop ends, word is {}\n",
        str::from_utf8(&word).unwrap()
    );

    Ok(())
}
