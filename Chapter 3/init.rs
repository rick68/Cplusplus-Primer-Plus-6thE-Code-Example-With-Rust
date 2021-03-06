// init.rs -- type changes on initialization

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let tree: f32 = 3 as f32;           // int converted to float
    let guess: i32 = 3.9832 as i32;     // double converted to int
    let debt: i32 = 7.2E12 as i32;      // result not defined in Rust

    write!(stdout, "tree = {:.*}\n", 6, tree);
    write!(stdout, "guess = {}\n", guess);
    write!(stdout, "debt = {}\n", debt);

    Ok(())
}
