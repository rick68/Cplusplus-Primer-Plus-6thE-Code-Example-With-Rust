// pointer.rs -- our first pointer variable

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut updates: i32 = 6;   // declare a variable
    let p_updates: *mut i32;    // declare pointer to an i32

    p_updates = &mut updates;   // assign address of i32 to pointer

    // express values two ways
    write!(stdout, "Values: update = {}", updates);
    write!(stdout, ", *p_updates = {}\n", unsafe { *p_updates });

    // express address two ways
    write!(stdout, "Address: &update = {:p}", &updates);
    write!(stdout, ", p_updates = {:p}\n", p_updates);

    // use pointer to change value
    unsafe {
        *p_updates = *p_updates + 1;
    }
    write!(stdout, "Now updates = {}\n", updates);

    Ok(())
}
