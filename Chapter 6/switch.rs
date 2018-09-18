// switch.rs -- using the match statement

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    showmenu();
    let mut choice: i32;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    choice = input.trim().parse::<i32>().unwrap_or(i32::default());
    while choice != 5 {
        match choice {
            1 => {
                let _: usize = stdout.write(b"\n")?;
                break;
            },
            2 => {
                report();
                break;
            },
            3 => {
                let _: usize = stdout.write(b"The boss was in all day.\n")?;
                break;
            },
            4 => {
                comfort();
                break;
            },
            _ => {
                let _: usize = stdout.write(b"That's not a choice.\n")?;
            },
        }
        showmenu();
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        choice = input.trim().parse::<i32>().unwrap_or(i32::default());
    };
    let _: usize = stdout.write(b"Bye!\n")?;

    Ok(())
}

fn showmenu() {
    let mut stdout: io::Stdout = io::stdout();
    stdout.write(b"Please enter 1, 2, 3, 4, or 5:\n").unwrap();
    stdout.write(b"1) alarm           2) report\n").unwrap();
    stdout.write(b"3) alibi           4) comfort\n").unwrap();
    stdout.write(b"5) quit\n").unwrap();
}

fn report() {
    let mut stdout: io::Stdout = io::stdout();
    stdout.write(b"It's been an excellent week for business.\n").unwrap();
    stdout.write(b"Sales are up 120% Expenses are down 35%.\n").unwrap();
}

fn comfort() {
    let mut stdout: io::Stdout = io::stdout();
    stdout.write(b"Your employees think you are the finest CEO.\n").unwrap();
    stdout.write(b"in the industry. The board of directors think.\n").unwrap();
    stdout.write(b"you are the finest CEO in the industry.\n").unwrap();
}
