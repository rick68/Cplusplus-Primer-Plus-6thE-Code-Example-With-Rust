// twoarg.rs -- a function with 2 arguments

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut times: usize = 0;
    let mut ch: u8 = 0;

    let _: usize = stdout.write(b"Enter a character: ")?;
    stdout.flush()?;

    let mut input: [u8; 1] = <[u8; 1]>::default();
    let mut token: String = String::new();
    'outter: loop {
        loop {
            match stdin.read(&mut input) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    if input[0] == 'q' as u8 {
                        break 'outter;
                    } else if input[0] == ' ' as u8 || input[0] == '\n' as u8 {
                        continue;
                    }
                    ch = input[0];
                    break;
                }
                Err(_) => break,
            }
        }
        let _: usize = stdout.write(b"Enter an integer: ")?;
        stdout.flush()?;
        token.clear();
        loop {
            match stdin.read(&mut input) {
                Ok(0) => break,
                Ok(_) => {
                    if input[0] != ' ' as u8 && input[0] != '\n' as u8 {
                        token.push(input[0] as char);
                    } else {
                        times = match token.parse::<usize>() {
                            Ok(val) => val,
                            Err(_) => continue,
                        };
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        n_chars(ch, times); // function with two arguments
        let _: usize = stdout.write(b"\nEnter another character or press the")?;
        let _: usize = stdout.write(b" q-key to quit: ")?;
        stdout.flush()?;
    }
    write!(stdout, "The value of times is {}.\n", times);
    let _: usize = stdout.write(b"Bye\n")?;

    Ok(())
}

fn n_chars(c: u8, n: usize) { // displays c n times
    let mut stdout: io::Stdout = io::stdout();
    for _ in 0..n {
        write!(stdout, "{}", c as char);
    }
}
