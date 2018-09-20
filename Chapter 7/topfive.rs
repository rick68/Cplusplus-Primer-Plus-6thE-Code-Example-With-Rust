// topfive.rs -- handling an array of string objects

use std::io;
use std::io::prelude::*;
use std::string::String;

const SIZE: usize = 5;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut list: [String; SIZE] = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ]; // an array holding 5 string object
    write!(stdout, "Enter you {} favorite astronomical sights:\n", SIZE);
    for i in 0..SIZE {
        write!(stdout, "{}: ", i + 1);
        stdout.flush()?;
        stdin.read_line(&mut list[i])?;
    }

    let _: usize = stdout.write(b"Your list:\n")?;
    display(&list, SIZE);

    Ok(())
}

fn display(sa: &[String], n: usize) {
    let mut stdout: io::Stdout = io::stdout();
    for i in 0..n {
        write!(stdout, "{}: {}", i + 1, sa[i]);
    }
}
