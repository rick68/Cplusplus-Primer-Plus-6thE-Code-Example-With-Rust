// secref.rs -- defining and using a reference

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
    write!(stdout, "rats address = {:p}", &rats);
    {
        let rodents: &i32 = &rats; // immutable borrow
        write!(stdout, ", rodents address = {:p}\n", rodents);
    }

    let bunnies: i32 = 50;
    {
        let rodents: &mut i32 = &mut rats; // mutable borrow
        *rodents = bunnies;
    }
    write!(stdout, "bunnies = {}", bunnies);
    write!(stdout, ", rats = {}", rats);
    {
        let rodents: &mut i32 = &mut rats; // mutable borrow
        write!(stdout, ", rodents = {}\n", rodents);
    }

    write!(stdout, "bunnies address = {:p}", &bunnies);
    {
        let rodents: &mut i32 = &mut rats; // mutable borrow
        write!(stdout, ", sodents address = {:p}\n", rodents);
    }

    Ok(())
}
