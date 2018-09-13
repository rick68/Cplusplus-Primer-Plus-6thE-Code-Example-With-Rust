// strings.rs -- storing strings in a array

use std::boxed::Box;
use std::io;
use std::io::prelude::*;
use std::mem::size_of_val;
use std::ptr;
use std::str;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    const SIZE: usize = 15;
    let mut name1: [u8; SIZE] = <[u8; SIZE]>::default(); // empty array
    let mut name2: Box<[u8]> = vec![0u8; SIZE].into_boxed_slice();
    unsafe { ptr::copy_nonoverlapping("rustowboy".as_ptr(), name2.as_mut_ptr(), 8) }; // initialized array

    write!(stdout, "Howdy! I'm {}", str::from_utf8(&name2).unwrap());
    stdout.write(b"! What's your name?\n")?;
    let n: usize = stdin.read(&mut name1)?;
    name1[n - 1] = 0; // remove newline
    write!(
        stdout,
        "Well, {}, your name has {} letters and is stored\n",
        str::from_utf8(&name1).unwrap(),
        n - 1
    );
    write!(stdout, "in an array of {} bytes.\n", size_of_val(&name1));
    write!(stdout, "Your initial is {}.\n", name1[0]);
    name2[4] = 0; // set to null character
    stdout.write(b"Here are the first 4 characters of my name: ")?;
    write!(stdout, "{}\n", str::from_utf8(&name2[..4]).unwrap());

    Ok(())
}
