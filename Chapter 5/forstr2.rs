// forstr2.rs -- reverseing an array

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let _: usize = stdout.write(b"Enter a word: ")?;
    let _: () = stdout.flush()?;
    let mut word: String;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    word = input.trim().to_string();

    // physically modify string object
    let mut temp: u8;
    {
        let v: &mut Vec<u8> = unsafe { word.as_mut_vec() };
        for (i, j) in (0..v.len()).rev().zip(0..) {     // start block
            if !(j < i) { break; }
            temp = v[i];
            v[i] = v[j];
            v[j] = temp;
        }                                               // end block
    }
    write!(stdout, "{}\nDone\n", word);

    Ok(())
}
