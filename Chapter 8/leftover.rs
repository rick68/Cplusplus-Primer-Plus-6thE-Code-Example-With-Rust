// leftover.rs -- overloading the left() function

use std::boxed::Box;
use std::io;
use std::io::prelude::*;
use std::string::String;
use std::vec::Vec;

macro_rules! left {
    ($arg1:expr; u64, $arg2:expr; u32) => (
        left_n_digits($arg1, $arg2)
    );
    ($arg1:expr; &[u8], $arg2:expr; usize) => (
        left_n_bytes!($arg1, $arg2)
    );
    ($arg1:expr; &[u8]) => (
        left_n_bytes!($arg1)
    );
}

macro_rules! left_n_bytes {
    ($s: expr, $n: expr) => {
        left_refu8slice_usize_boxvecu8($s, $n)
    };
    ($s: expr) => {
        left_refu8slice_usize_boxvecu8($s, 1);
    };
}

// This function returns the first ct digits of the number num.
fn left_n_digits(mut num: u64, mut ct: u32) -> u64 {
    let mut digits: u32 = 1;
    let mut n: u64 = num;

    if ct == 0 || num == 0 {
        return 0;  // return 0 if no digits
    }
    loop {
        n /= 10;
        if n > 0 {
            digits += 1;
        } else {
            break;
        }
    }
    if digits > ct {
        ct = digits - ct;
        for _ in 0..ct {
            num /= 10;
        }
        num     // return left ct digits
    } else {    // if ct >= number of digits
        num     // return the whole number
    }
}

// This function returns a pointer to a new string
// consisting of the first n characters in the s string slice.
fn left_refu8slice_usize_boxvecu8(s: &[u8], n: usize) -> Box<Vec<u8>> {
    let mut p: Box<Vec<u8>> = Box::new(vec![0; n]);
    for i in 0..n {
        if i >= s.len() || s[i] == 0 {
            break;
        }
        p[i] = s[i]; // copy characters
    }
    p
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let trip: &str = "Hawaii!!";        // test value
    let n: u64 = 12345678;              // test value
    let mut temp: Box<Vec<u8>>;

    for i in 1..10 {
        write!(stdout, "{}\n", left!(n; u64, i as u32; u32));
        temp = left!(trip.as_bytes(); &[u8], i; usize);
        write!(stdout, "{}\n", String::from_utf8(temp.to_vec()).unwrap());
    }

    Ok(())
}
