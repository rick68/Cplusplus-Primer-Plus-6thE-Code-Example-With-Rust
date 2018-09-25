// swaps.rs -- swapping with references and with pointers

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut wallet1: i32 = 300;
    let mut wallet2: i32 = 350;

    write!(stdout, "wallet1 = ${}", wallet1);
    write!(stdout, " wallet2 = ${}\n", wallet2);

    let _: usize = stdout.write(b"Using references to swap contents:\n")?;
    swapr(&mut wallet1, &mut wallet2);   // pass variables
    write!(stdout, "wallet1 = ${}", wallet1);
    write!(stdout, " wallet2 = ${}\n", wallet2);

    let _: usize = stdout.write(b"Using pointers to swap contents again:\n")?;
    swapp(&mut wallet1, &mut wallet2);   // pass address of variables
    write!(stdout, "wallet1 = ${}", wallet1);
    write!(stdout, " wallet2 = ${}\n", wallet2);

    let _: usize = stdout.write(b"Trying to use passing by value:\n")?;
    swapv(wallet1, wallet2);            // pass values of variables
    write!(stdout, "wallet1 = ${}", wallet1);
    write!(stdout, " wallet2 = ${}\n", wallet2);

    Ok(())
}

fn swapr(a: &mut i32, b: &mut i32) {
    let temp: i32;

    temp = *a;  // use *a, *b for values of variables
    *a = *b;
    *b = temp;
}

fn swapp(p: *mut i32, q: *mut i32) {
    let temp: i32;

    unsafe {
        temp = *p;  // use *p, *q for values of variables
        *p = *q;
        *q = temp;
    }
}

fn swapv(mut a: i32, mut b: i32) {
    let temp: i32;

    temp = a;   // use a, b for values of variables
    a = b;
    b = temp;
}
