// stacker.rs -- testing the Stack struct

mod stack;
use stack::*;

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut st: Stack = Stack::new(); // create an empty stack
    let mut ch: [u8; 1] = <[u8; 1]>::default();
    let mut po: Item = Item::default();
    let mut token: String = String::new();
    'outer: loop {
        let _: usize = stdout.write(b"Please enter A to add a purchase order,\n")?;
        let _: usize = stdout.write(b"P to process a PO, or Q to quit.\n")?;
        token.clear();
        loop {
            match stdin.read(&mut ch) {
                Ok(0) => break 'outer,  // EOF
                Ok(_) => {
                    if token.is_empty() {
                        if ch[0].to_ascii_uppercase() == 'Q' as u8 {
                            break 'outer;
                        } else if ch[0] != ' ' as u8 && ch[0] != '\n' as u8 {
                            token.push(ch[0] as char);
                            continue;
                        }
                    } else {
                        if ch[0] == '\n' as u8 {
                            break;
                        }
                    }
                },
                Err(_) => break 'outer,
            }
        }
        if ch[0].is_ascii_alphabetic() {
            continue;
        }
        match token.chars().next().unwrap() {
            'A' | 'a' => {
                let _: usize = stdout.write(b"Enter a PO number to add: ")?;
                stdout.flush()?;
                token.clear();
                loop {
                    match stdin.read(&mut ch) {
                        Ok(0) => break, // EOF
                        Ok(_) => {
                            if ch[0] != ' ' as u8 && ch[0] != '\n' as u8 {
                                token.push(ch[0] as char);
                            } else if !token.is_empty() {
                                break;
                            }
                        },
                        Err(_) => break,
                    }
                }
                po = token.parse::<Item>().unwrap_or(Item::default());
                if st.isfull() {
                    let _: usize = stdout.write(b"stack already full\n")?;
                } else {
                    st.push(&po);
                }
            },
            'P' | 'p' => {
                if st.isempty() {
                    let _: usize = stdout.write(b"stack already empty\n")?;
                } else {
                    st.pop(&mut po);
                    write!(stdout, "PO #{} popped\n", po);
                }
            },
            _ => (),
        }
    }

    Ok(())
}
