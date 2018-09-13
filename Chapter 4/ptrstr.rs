// ptrstr.rs -- using pointer to strings

use std::boxed::Box;
use std::io;
use std::io::prelude::*;
use std::slice;
use std::str;
use std::string::String;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut animal: String = String::from("bear"); // animal holds bear
    let bird: *const u8 = "wren".as_ptr(); // bird holds address of string
    let mut ps: *const u8; // uninitialized

    write!(stdout, "{} and", animal); // display bear
    write!(
        stdout,
        " {}...\n",
        str::from_utf8(unsafe { slice::from_raw_parts(bird, 4) }).unwrap()
    ); // display wren

    stdout.write(b"Enter a kind of animal: ")?;
    stdout.flush()?;
    animal.clear();
    let n: usize = stdin.read_line(&mut animal)?;

    ps = animal.as_ptr();
    write!(
        stdout,
        "{}!\n",
        str::from_utf8(unsafe { slice::from_raw_parts(ps, n - 1) }).unwrap()
    ); // ok, same as using animal
    write!(stdout, "Before using pointer::copy_to_nonoverlapping():\n");
    write!(stdout, "{} at {:p}\n", animal.trim(), animal.as_ptr());
    write!(
        stdout,
        "{} at {:p}\n",
        str::from_utf8(unsafe { slice::from_raw_parts(ps, n - 1) }).unwrap(),
        ps
    );

    ps = unsafe { &mut *Box::into_raw(vec![0u8; animal.len()].into_boxed_slice()) }.as_ptr(); // get new storage
    unsafe {
        animal
            .as_ptr()
            .copy_to_nonoverlapping(ps as *mut u8, animal.len()) // copy string to new storage
    };
    stdout.write(b"After using pointer::copy_to_nonoverlapping():\n")?;
    write!(stdout, "{} at {:p}\n", animal.trim(), animal.as_ptr());
    write!(
        stdout,
        "{} at {:p}\n",
        str::from_utf8(unsafe { slice::from_raw_parts(ps, animal.len() - 1) }).unwrap(),
        ps
    );
    unsafe { Box::from_raw(ps as *mut Vec<u8>) };

    Ok(())
}
