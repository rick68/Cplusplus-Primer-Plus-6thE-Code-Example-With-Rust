// left.rs -- using function with a default argument

use std::boxed::Box;
use std::io;
use std::io::prelude::*;
use std::string::String;
use std::vec::Vec;

const ARSIZE: usize = 80;

macro_rules! left {
    ($s: expr, $n: expr) => {
        left($s, $n);
    };
    ($s: expr) => {
        left($s, 1);
    };
}

// This function returns a pointer to a new string
// consisting of the first n characters in the s string slice.
fn left(s: &[u8], n: usize) -> Box<Vec<u8>> {
    let mut p: Box<Vec<u8>> = Box::new(vec![0; n]);
    for i in 0..n {
        if s[i] == 0 {
            break;
        }
        p[i] = s[i]; // copy characters
    }
    p
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut sample: [u8; ARSIZE] = [0; ARSIZE];
    let _: usize = stdout.write(b"Enter a string:\n")?;
    let n: usize = stdin.read(&mut sample)?;
    sample[n - 1] = 0; // remove newline
    let mut ps: Box<Vec<u8>> = left!(&sample, 4);
    write!(stdout, "{}\n", String::from_utf8(*ps).unwrap());
    ps = left!(&sample);
    write!(stdout, "{}\n", String::from_utf8(*ps).unwrap());

    Ok(())
}
