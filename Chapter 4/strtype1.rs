// strtype1.rs -- using the rust String struct

use std::io;
use std::io::prelude::*;
use std::ptr;
use std::str;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut charr1: [u8; 20] = <[u8; 20]>::default(); // create an empty array
    let mut charr2: [u8; 20] = <[u8; 20]>::default(); // create an empty array
    unsafe { ptr::copy_nonoverlapping("jaguar".as_ptr(), (&mut charr2).as_mut_ptr(), 6) }; // initialized array

    let mut str1: String = String::new(); // create an empty string object
    let str2: String = String::from("panther"); // create an initialized string

    stdout.write(b"Enter a kind of feline: ")?;
    stdout.flush()?;
    let n: usize = stdin.read(&mut charr1)?;
    charr1[n - 1] = 0; // remove newline
    stdout.write(b"Enter another kind of feline: ")?;
    stdout.flush()?;
    stdin.read_line(&mut str1)?;
    stdout.write(b"Here are some felines:\n")?;
    write!(
        stdout,
        "{} {} {} {}\n",
        str::from_utf8(&charr1).unwrap(),
        str::from_utf8(&charr2).unwrap(),
        str1.trim(),
        str2.trim()
    );
    write!(
        stdout,
        "The third letter in {} is {}\n",
        str::from_utf8(&charr2).unwrap(),
        char::from(charr2[2])
    );
    write!(
        stdout,
        "The third letter in {} is {}\n",
        str2,
        char::from(str2.as_bytes()[2])
    );

    Ok(())
}
