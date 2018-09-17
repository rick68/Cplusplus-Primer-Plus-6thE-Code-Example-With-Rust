// or.rs -- using the logical OR operator

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut ch: [u8; 1] = <[u8; 1]>::default();
    let _: usize = stdout.write(b"This program may reformat your hard disk\n")?;
    let _: usize = stdout.write(b"and destroy all your data.\n")?;
    let _: usize = stdout.write(b"Do you wish to continue? <y/n> ")?;
    let _: () = stdout.flush()?;
    let _: () = stdin.read_exact(&mut ch)?;
    if ch[0] == 'y' as u8 || ch[0] == 'Y' as u8 {               // y or Y
        let _: usize = stdout.write(b"You were warned!\n")?;
    } else if ch[0] == 'n' as u8 || ch[0] == 'N' as u8 {        // n or N
        let _: usize = stdout.write(b"A wish choice ... bye\n")?;
    } else {
        let _: usize = stdout.write(b"That wasn't a y or n! Apparently you ")?;
        let _: usize = stdout.write(b"can't follow\ninstructions, so ")?;
        let _: usize = stdout.write(b"I'll trash your disk anyway.\n")?;
    }

    Ok(())
}
