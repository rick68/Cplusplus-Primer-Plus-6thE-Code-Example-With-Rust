// arraynew.rs -- using the Box::new() for arrays

use std::boxed::Box;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut p3: Box<[f64; 3]> = Box::new(<[f64; 3]>::default()); // space for 3 f64
    p3[0] = 0.2; // treat p3 like an array name
    p3[1] = 0.5;
    p3[2] = 0.8;
    write!(stdout, "p3[1] is {}.\n", p3[1]);
    let p3: *mut [f64; 3] = Box::into_raw(p3);
    let p3: *const f64 = p3 as *const f64;
    let p3: *const f64 = unsafe { p3.offset(1) }; // increment the pointer
    let p3: Box<[f64; 2]> = unsafe { Box::from_raw(p3 as *mut [f64; 2]) };
    write!(stdout, "Now p3[0] is {} and ", p3[0]);
    write!(stdout, "p3[1] is {}.\n", p3[1]);
    let p3: *mut [f64; 2] = Box::into_raw(p3);
    let p3: *const f64 = p3 as *const f64;
    let p3: *const f64 = unsafe { p3.offset(-1) }; // point back to beginning
    unsafe { Box::from_raw(p3 as *mut [f64; 3]) }; // memory will be free by drop()

    Ok(())
}
