// arfupt.rs -- an array of function pointers

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let av: [f64; 3] = [1112.3, 1542.6, 2227.9];

    // pointer to a function
    let p1: fn(&[f64], usize) -> &[f64] = f1;
    let p2 = f2; // automatic type deduction
    let _: usize = stdout.write(b"Using pointers to functions:\n")?;
    let _: usize = stdout.write(b"Address Value\n")?;
    write!(stdout, "{:p}: {}\n", p1(&av[..], 3), p1(&av[..], 3)[0]);
    write!(stdout, "{:p}: {}\n", p2(&av[..], 3), p2(&av[..], 3)[0]);

    // pa an array of pointers
    let pa: [fn(&[f64], usize) -> &[f64]; 3] = [f1, f2, f3];
    // pa a pointer to first element of pa
    let pb = pa;
    let _: usize = stdout.write(b"\nUsing an array of pointers to functions:\n")?;
    let _: usize = stdout.write(b" Address Value\n")?;
    for i in 0..3 {
        write!(stdout, "{:p}: {}\n", pb[i](&av[..], 3), pb[i](&av[..], 3)[0]);
    }

    // what about a pointer to an array of function pointers
    let _: usize = stdout.write(b"\nUsing pointers to an array of pointers:\n")?;
    let _: usize = stdout.write(b" Address Value\n")?;
    // easy way to declare pc
    let pc = &pa;
    write!(stdout, "{:p}: {}\n", pc[0](&av[..], 3), pc[0](&av[..], 3)[0]);
    // hard way to declare pd
    let pd: &[fn(&[f64], usize) -> &[f64]; 3] = &pa;
    // store return value in pdb
    let pdb: &[f64] = pd[1](&av[..], 3);
    write!(stdout, "{:p}: {}\n", pdb, pdb[0]);
    // alternative notation
    write!(stdout, "{:p}: {}\n", pd[2](&av[..], 3), pd[2](&av[..], 3)[0]);

    Ok(())
}

// some rather dull functions

#[allow(unused)]
fn f1(ar: &[f64], n: usize) -> &[f64] {
    ar
}

fn f2(ar: &[f64], n: usize) -> &[f64] {
    assert!(n >= 1);
    &ar[1..]
}

fn f3(ar: &[f64], n: usize) -> &[f64] {
    assert!(n >= 2);
    &ar[2..]
}
