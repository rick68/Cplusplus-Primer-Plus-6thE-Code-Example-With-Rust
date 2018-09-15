// strtype2.rs -- assigning, adding, and appending

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut s1: String = String::from("penguin");
    let mut s2: String = String::new();
    let mut s3: String = String::new();

    stdout.write(b"You can assign one string object to another: s2 = s1.clone()\n")?;
    s2 = s1.clone();
    write!(stdout, "s1: {}, s2: {}\n", s1, s2);
    stdout.write(b"You can assign a str to a string object.\n")?;
    write!(stdout, "s2 = \"buzzard\".to_string()\n");
    s2 = "buzzard".to_string();
    write!(stdout, "s2: {}\n", s2);
    write!(
        stdout,
        "You can concatenate strings: s3 = sl.clone() + s2.as_str()\n"
    );
    s3 = s1.clone() + s2.as_str();
    write!(stdout, "s3: {}", s3);
    stdout.write(b"You can append strings.\n")?;
    s1 += s2.as_str();
    write!(stdout, "s1 += s2.as_str() yields s1 = {}\n", s1);
    s2 += " for a day";
    write!(stdout, "s2 += \" for a day\" yields s2 = {}\n", s2);

    Ok(())
}
