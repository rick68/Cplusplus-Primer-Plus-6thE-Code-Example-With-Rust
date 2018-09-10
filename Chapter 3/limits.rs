// limits.rs -- some integer limits

use std::io;
use std::io::prelude::*;
use std::i32;
use std::i16;
use std::i64;
use std::i128;
use std::i8;
use std::mem::size_of;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let n_i32: i32 = i32::MAX;  // initialize n_i32 to max int value
    let n_i16: i16 = i16::MAX;
    let n_i64: i64 = i64::MAX;
    let n_i128: i128 = i128::MAX;

    write!(stdout, "i32 is {} bytes.\n", size_of::<i32>());
    write!(stdout, "i16 is {} bytes.\n", size_of::<i16>());
    write!(stdout, "i64 is {} bytes.\n", size_of::<i64>());
    write!(stdout, "i128 is {} bytes.\n", size_of::<i128>());
    stdout.write(b"\n")?;

    stdout.write(b"Maximum values:\n")?;
    write!(stdout, "i32: {}\n", n_i32);
    write!(stdout, "i16: {}\n", n_i16);
    write!(stdout, "i64: {}\n", n_i64);
    write!(stdout, "i128: {}\n", n_i128);
    stdout.write(b"\n")?;

    write!(stdout, "Minimum i32 values = {}\n", i32::MIN);
    write!(stdout, "Bits per byte = {}\n", size_of::<i8>() * 8);

    Ok(())
}
