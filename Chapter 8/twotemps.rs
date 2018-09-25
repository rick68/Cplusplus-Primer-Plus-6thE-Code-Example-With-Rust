// twotemps.rs -- using overloaded template functions

use std::io;
use std::io::prelude::*;
use std::clone::Clone;
use std::marker::Copy;

macro_rules! swap {
    ($a:expr; &mut T, $b:expr; &mut T) => (
        swap($a, $b)
    );
    ($a:expr; &mut [T], $b:expr; &mut [T], $n:expr; usize) => (
        swap_array($a, $b, $n)
    );
}

const LIM: usize = 8;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut i: i32 = 10;
    let mut j: i32 = 20;

    write!(stdout, "i, j = {}, {}.\n", i, j);
    let _: usize = stdout.write(b"Using compiler-generated i32 swapper:\n")?;
    swap!(&mut i; &mut T, &mut j; &mut T);
    write!(stdout, "Now i, j = {}, {}.\n", i, j);

    let mut d1: [i32; LIM] = [0, 7, 0, 4, 1, 7, 7, 6];
    let mut d2: [i32; LIM] = [0, 7, 2, 0, 1, 9, 6, 9];
    let _: usize = stdout.write(b"Original arrays:\n")?;
    show(&d1);
    show(&d2);
    swap!(&mut d1; &mut [T], &mut d2; &mut [T], LIM; usize);    // matches new template
    let _: usize = stdout.write(b"Swapped arrays:\n")?;
    show(&d1);
    show(&d2);

    Ok(())
}

fn swap<T: Clone + Copy>(a: &mut T, b: &mut T) {
    let temp: T;
    temp = *a;
    *a = *b;
    *b = temp;
}

fn swap_array<T: Clone + Copy>(a: &mut [T], b: &mut [T], n: usize) {
    let mut temp: T;
    for i in 0..n {
        temp = a[i];
        a[i] = b[i];
        b[i] = temp;
    }
}

fn show(a: &[i32]) {
    let mut stdout: io::Stdout = io::stdout();

    write!(stdout, "{}{}/", a[0], a[1]);
    write!(stdout, "{}{}/", a[2], a[3]);
    for i in 4..LIM {
        write!(stdout, "{}", a[i]);
    }
    let _: usize = stdout.write(b"\n").unwrap();
}
