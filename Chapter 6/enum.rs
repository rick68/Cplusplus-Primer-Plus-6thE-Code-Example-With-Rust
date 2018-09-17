// enum.rs -- using enum

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
use std::string::String;

// create named constants for 0 - 6
#[allow(unused)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Indigo,
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let _: usize = stdout.write(b"Enter color code (0-6): ")?;
    let _: () = stdout.flush()?;
    let mut code: i32 = 0;
    let mut input: String = String::new();
    loop {
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        code = match input.trim().parse::<i32>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        match code {
            0 => {
                let _: usize = stdout.write(b"Her lips were red.\n")?;
            }
            1 => {
                let _: usize = stdout.write(b"Her hair was orange.\n")?;
            }
            2 => {
                let _: usize = stdout.write(b"Her shoes were yellow.\n")?;
            }
            3 => {
                let _: usize = stdout.write(b"Her nails were green.\n")?;
            }
            4 => {
                let _: usize = stdout.write(b"Her sweatsuit was blue.\n")?;
            }
            5 => {
                let _: usize = stdout.write(b"Her eyes were violet.\n")?;
            }
            6 => {
                let _: usize = stdout.write(b"Her mood was indigo.\n")?;
            }
            _ => break,
        }
        let _: usize = stdout.write(b"Enter color code (0-6): ")?;
        let _: () = stdout.flush()?;
    }
    let _: usize = stdout.write(b"Bye\n")?;

    Ok(())
}
