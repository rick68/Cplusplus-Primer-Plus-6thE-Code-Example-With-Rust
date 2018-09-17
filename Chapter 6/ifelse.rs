// ifelse.rs -- using the if else statement

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut ch: [u8; 1] = <[u8; 1]>::default();
    let _: usize = stdout.write(b"Type, and I shall repeat.\n")?;
    let _: () = stdin.read_exact(&mut ch)?;
    while ch[0] != '.' as u8 {
        if ch[0] == '\n' as u8 {
            let _: usize = stdout.write(&mut ch)?; // done if newline
        } else {
            ch[0] += 1;
            let _: usize = stdout.write(&mut ch)?; // done otherwise
        }
        let _: () = stdin.read_exact(&mut ch)?;
    }
    let _: usize = stdout.write(b"\nPlease excuse the slight confusion.\n")?;

    Ok(())
}
