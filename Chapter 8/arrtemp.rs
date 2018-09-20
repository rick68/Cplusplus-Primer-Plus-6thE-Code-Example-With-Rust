// arrtemp.rs -- using template functions with array template

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::marker::Copy;
use std::fmt::Display;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let ai: [i32; 5] = [1, 2, 3, 4, 5];
    let astr: [&str; 5] = [
        "string under construction",
        "stupid string indded",
        "there's nothing to see",
        "nothing to do",
        "but enjoy all that is",
    ];
    display(&ai);
    display(&astr);

    Ok(())
}

fn display<T: Copy + Display>(ar: &[T]) {
    let mut stdout: io::Stdout = io::stdout();
    for i in ar {
        write!(stdout, "{}\n", i);
    }
}
