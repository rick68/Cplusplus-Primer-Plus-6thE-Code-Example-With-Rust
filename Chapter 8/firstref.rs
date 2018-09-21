// firstref.rs -- defining and using a reference

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut rats: i32 = 101;

    write!(stdout, "rats = {}", rats);
    {
        let rodents: &i32 = &rats; // immutable borrow
        write!(stdout, ", rodents = {}\n", rodents);
    }
    {
        let rodents: &mut i32 = &mut rats; // mutable borrow
        *rodents += 1;
    }
    write!(stdout, "rats = {}", rats);
    {
        let rodents: &i32 = &rats; // immutable borrow
        write!(stdout, ", rodents = {}\n", rodents);
    }

    write!(stdout, "rats address = {:p}", &rats);
    {
        let rodents: &i32 = &rats; // immutable borrow
        write!(stdout, ", rodents address = {:p}\n", rodents);
    }

    Ok(())
}
