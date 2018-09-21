// funtemp.rs -- using a function template

use std::io;
use std::io::prelude::*;
use std::clone::Clone;
use std::marker::Copy;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut i: i32 = 10;
    let mut j: i32 = 20;
    write!(stdout, "i, j = {}, {}.\n", i, j);
    let _: usize = stdout.write(b"Using compiler-generated i32 swapper:\n")?;
    swap(&mut i, &mut j); // generates swap(a: &mut i32, b: &mut i32)
    write!(stdout, "Now i, j = {}, {}.\n", i, j);

    let mut x: f64 = 24.5;
    let mut y: f64 = 81.7;
    write!(stdout, "x, y = {}, {}.\n", x, y);
    let _: usize = stdout.write(b"Using compiler-generated i64 swapper:\n")?;
    swap(&mut x, &mut y); // generates swap(a: &mut f64, b: &mut f64)
    write!(stdout, "Now x, y = {}, {}.\n", x, y);

    Ok(())
}

// function template definition

fn swap<T>(a: &mut T, b: &mut T)
    where T: Clone + Copy
{
    let temp: T = *a;    // temp a variable of type T
    *a = *b;
    *b = temp;
}
