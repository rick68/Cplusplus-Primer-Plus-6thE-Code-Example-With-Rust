// testin3.rs -- reading chars to end of file

use std::io;
use std::io::prelude::*;
use std::io::Take;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut ch: [u8; 1] = <[u8; 1]>::default();
    let mut count: i32 = 0;                     // use basic input

    let _: usize = stdout.write(b"Enter characters; enter # to quit:\n")?;
    let mut handle: Take<io::Stdin> = stdin.take(1);
    let _: usize = handle.read(&mut ch)?;       // get a character
    'while_loop: while ch[0] != '#' as u8 {     // test the character
        write!(stdout, "{}", ch[0] as char);    // echo the character
        let _: () = stdout.flush()?;
        count += 1;                             // count the character
        loop {
            handle.set_limit(1);
            let n: usize = handle.read(&mut ch)?;       // get the next character
            if n == 0 { break 'while_loop; }    // EOF
            if ch[0] == '\n' as u8 { continue; }
            break;
        }
    }
    write!(stdout, "\n{} characters read\n", count);

    Ok(())
}
