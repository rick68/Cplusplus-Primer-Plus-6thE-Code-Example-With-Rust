// strgfun.rs -- functions with a string argument

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mmm: &str = "minimum";  // string slice
    let wail: &str = "ululate"; // string slice

    let ms: usize = c_in_str(mmm, 'm' as u8);
    let us: usize = c_in_str(wail, 'u' as u8);

    write!(stdout, "{} m characters in {}\n", ms, mmm);
    write!(stdout, "{} u characters in {}\n", us, wail);

    Ok(())
}

// this function counts the number of ch characters
// in the string str
fn c_in_str(s: &str, ch: u8) -> usize {
    let mut count: usize = 0;
    for c in s.bytes() {
        if c == ch {
            count += 1;
        }
    }
    count
}
