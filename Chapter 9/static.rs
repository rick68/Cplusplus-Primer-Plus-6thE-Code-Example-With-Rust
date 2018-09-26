// static.rs -- using a static local variable

use std::io;
use std::io::prelude::*;
use std::str;

const ARSIZE: usize = 10;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut input: [u8; ARSIZE] = <[u8; ARSIZE]>::default();
    let mut next: [u8; 1] = <[u8; 1]>::default();

    let _: usize = stdout.write(b"Enter a line:\n")?;
    let mut n: usize;
    loop {
        n = stdin.read(&mut input)?;
        if n == 1 && input[0] == '\n' as u8 {
            break;
        } else if n == ARSIZE && input[n - 1] != '\n' as u8 {
            loop {                              // string didn't fit!
                match stdin.read(&mut next) {   // dispose of remainder
                    Ok(_) => {
                        if next[0] == '\n' as u8 {
                            break;
                        }
                    },
                    Err(_) => break,
                }
            }
        }
        strcount(&str::from_utf8(&input[0..n-1]).unwrap());
        let _: usize = stdout.write(b"Enter next line (empty line to quit):\n")?;
    }
    let _: usize = stdout.write(b"Bye\n")?;

    Ok(())
}

fn strcount(s: &str) {
    let mut stdout: io::Stdout = io::stdout();
    static mut TOTAL: usize = 0;
    let count: usize;

    write!(stdout, "\"{}\" contains ", s);
    count = s.len();
    unsafe { TOTAL += count; }
    write!(stdout, "{} characters\n", count);
    write!(stdout, "{} characters total\n", unsafe { TOTAL });

}