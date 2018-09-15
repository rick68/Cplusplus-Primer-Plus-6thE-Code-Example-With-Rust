// strtype3.rs -- more string object features

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
use std::ptr;
use std::string::String;
use std::str;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut charr1: [u8; 20] = <[u8; 20]>::default();
    let mut charr2: [u8; 20] = <[u8; 20]>::default();
    unsafe { ptr::copy_nonoverlapping("jaguar".as_ptr(), charr2.as_mut_ptr(), 6) };
    let mut str1: String = String::new();
    let str2: String = String::from("panther");

    // assignment for string objects and u8 arrays
    str1 = str2.clone(); // copy str2 to str1
    unsafe { ptr::copy_nonoverlapping(charr2.as_ptr(), charr1.as_mut_ptr(), 6) }; // copy charr2 to charr1

    // appending for string objects and u8 arrays
    str1 += " paste"; // add paste to end of str1
    unsafe { ptr::copy_nonoverlapping(" juice".as_ptr(), charr1.as_mut_ptr().offset(6), 6) }; // copy charr2 to charr1

    // finding the length of a string object and a str
    let len1: usize = str1.len(); // obtain length of str1
    let len2: usize = str::from_utf8(&charr1).unwrap().len(); // obtain length of charr1

    write!(
        stdout,
        "The string {} contains {} characters.\n",
        str1, len1
    );
    write!(
        stdout,
        "The string {} contains {} characters.\n",
        str::from_utf8(&charr1).unwrap(),
        len2
    );

    Ok(())
}
