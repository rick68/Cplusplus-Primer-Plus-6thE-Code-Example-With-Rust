// tempover.rs -- template overloading

use std::io;
use std::io::prelude::*;
use std::str;
use std::ptr;
use std::fmt::Display;

struct Debts {
    #[allow(unused)]
    name: &'static str,
    amount: f64,
}

macro_rules! show_array {
    ($arr:expr; &[T], $n:expr; usize) => (
        show_array($arr, $n);
    );
    ($arr:expr; *const [*const T], $n:expr; usize) => (
        show_array_by_pointer($arr, $n);
    );
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let things: [i32; 6] = [13, 31, 103, 301, 310, 130];
    let mr_e: [Debts; 3] = [
        Debts {
            name: "Ima Wolfe",
            amount: 2400.0,
        },
        Debts {
            name: "Ura Foxe",
            amount: 1300.0,
        },
        Debts {
            name: "Iby Stout",
            amount: 1800.0,
        },
    ];
    let mut pd: [*const f64; 3] = [ptr::null(); 3];

    // set immutable borrow to the amount members of the structures in mr_e
    for i in 0..3 {
        pd[i] = &mr_e[i].amount;
    }

    stdout.write(b"Listing Mr. E's counts of things:\n")?;
    // things is an array of i32
    show_array!(&things; &[T], 6; usize); // uses template A
    stdout.write(b"Listing Mr. E's debts:\n")?;
    // pd is an array of pointers to f64
    show_array!(&pd; *const [*const T], 3; usize);

    Ok(())
}

fn show_array<T: Display>(arr: &[T], n: usize) {
    let mut stdout: io::Stdout = io::stdout();

    let _: usize = stdout.write(b"template A\n").unwrap();
    for i in 0..n {
        write!(stdout, "{} ", arr[i]);
    }
    let _: usize = stdout.write(b"\n").unwrap();
}

fn show_array_by_pointer<T: Display>(arr: *const [*const T], n: usize) {
    let mut stdout: io::Stdout = io::stdout();

    let _: usize = stdout.write(b"template B\n").unwrap();
    for i in 0..n {
        unsafe {
            write!(stdout, "{} ", *(*arr)[i]);
        }
    }
    let _: usize = stdout.write(b"\n").unwrap();
}
