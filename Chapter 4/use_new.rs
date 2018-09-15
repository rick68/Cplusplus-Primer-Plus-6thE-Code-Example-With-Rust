// use_new.rs -- using the Box::new()

use std::boxed::Box;
use std::io;
use std::io::prelude::*;
use std::mem::size_of_val;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let nights: i32 = 1001;
    let mut pt: Box<i32> = Box::new(i32::default()); // allocate space for an i32
    *pt = 1001; // store a value there

    stdout.write(b"nights value = ")?;
    write!(stdout, "{}: location {:p}\n", nights, &nights);
    stdout.write(b"i32 ")?;
    write!(stdout, "value = {}: location = {:p}\n", *pt, pt);

    let mut pd: Box<f64> = Box::new(f64::default()); // allocate space for a f64
    *pd = 10_000_001.0; // store a f64 here

    stdout.write(b"double ")?;
    write!(stdout, "value = {}: location = {:p}\n", *pd, pd);
    write!(stdout, "location of pointer pd: {:p}\n", &pd);
    write!(stdout, "size of pt = {}", size_of_val(&pt));
    write!(stdout, ": size of *pt = {}\n", size_of_val(&*pt));
    write!(stdout, "size of pd = {}", size_of_val(&pd));
    write!(stdout, ": size of *pd = {}\n", size_of_val(&*pd));

    Ok(())
}
