// choices.rs -- choosing a template

#![feature(core_intrinsics)]

use std::clone::Clone;
use std::cmp::PartialOrd;
use std::default::Default;
use std::intrinsics::type_name;
use std::io;
use std::io::prelude::*;
use std::marker::Copy;
use std::ops::Neg;

fn lesser<T>(a: T, b: T) -> T
where
    T: PartialOrd + Neg<Output = T> + Default + Copy + Clone,
{
    match unsafe { type_name::<T>() } {
        "i32" => {
            let a = if a < T::default() { -a } else { a };
            let b = if b < T::default() { -b } else { b };
            if a < b { a } else { b }
        } // #2
        _ => if a < b { a } else { b }, // #1
    }
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let m: i32 = 20;
    let n: i32 = -30;
    let x: f64 = 15.5;
    let y: f64 = 25.9;

    write!(stdout, "{}\n", lesser(m, n));                       // use #2
    write!(stdout, "{}\n", lesser(x, y));                       // use #1
    write!(stdout, "{}\n", lesser(m, n));                       // use #1 with i32
    write!(stdout, "{}\n", lesser::<i32>(x as i32, y as i32));  // use #1 with i32

    Ok(())
}
