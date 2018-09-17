// jump.rs -- using continue and break

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut spaces: i32 = 0;

    let _: usize = stdout.write(b"Enter a line of text:\n")?;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    let line: &[u8] = input.trim().as_bytes();
    write!(stdout, "Complete line:\n{}\n", input.trim());
    let _: usize = stdout.write(b"Line through first period:\n")?;
    for i in 0..line.len() {
        if !(line[i] != '\0' as u8) {
            break;
        }
        write!(stdout, "{}", line[i] as char);  // display character
        if line[i] == '.' as u8 {               // quit if it's a period
            break;
        }
        if line[i] == ' ' as u8 {               // skip rest of loop
            continue;
        }
        spaces += 1;
    }
    write!(stdout, "\n{} spaces\n", spaces);
    let _: usize = stdout.write(b"Done\n")?;

    Ok(())
}
