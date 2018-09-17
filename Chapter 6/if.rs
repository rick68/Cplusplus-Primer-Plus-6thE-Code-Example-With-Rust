// if.rs -- using the if statement

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut ch: [u8; 1] = <[u8; 1]>::default();
    let mut spaces: i32 = 0;
    let mut total: i32 = 0;
    let _: () = stdin.read_exact(&mut ch)?;
    while ch[0] != '.' as u8 {  // quit at end of sentence
        if ch[0] == ' ' as u8 { // check if ch is a space
            spaces += 1;
        }
        total += 1; // done every time
        let _: () = stdin.read_exact(&mut ch)?;
    }
    write!(stdout, "{} spaces, {}", spaces, total);
    let _: usize = stdout.write(b" characters total in sentence\n")?;

    Ok(())
}
