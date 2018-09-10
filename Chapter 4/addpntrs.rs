// addpntrs.rs -- pointer addition

use std::io;
use std::io::prelude::*;
use std::mem::size_of_val;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let wages: [f64; 3] = [10000.0, 2000.0, 3000.0];
    let stacks: [i16; 3] = [3, 2, 1];

    // Here are two ways to get the address of an array
    let pw: *const f64 = &wages as *const f64; // name of an array = address
    let ps: *const i16 = &stacks as *const i16; // or use address operator
    // with array element
    write!(stdout, "pw = {:p}, *pw = {}\n", pw, unsafe { *pw });
    let pw: *const f64 = unsafe { pw.offset(1) };
    stdout.write(b"add 1 to the pw pointer:\n")?;
    write!(stdout, "pw = {:p}, *pw = {}\n\n", pw, unsafe { *pw });

    write!(stdout, "ps = {:p}, *ps = {}\n", ps, unsafe { *ps });
    let ps: *const i16 = unsafe { ps.offset(1) };
    stdout.write(b"add 1 to the ps pointer:\n")?;
    write!(stdout, "ps = {:p}, *ps = {}\n\n", ps, unsafe { *ps });

    stdout.write(b"access two elements with array notation\n")?;
    write!(stdout, "stacks[0] = {}, stacks[1] = {}\n", stacks[0], stacks[1]);

    write!(stdout, "{} = size of wages array\n", size_of_val(&wages));
    write!(stdout, "{} = size of pw pointer\n", size_of_val(&pw));

    Ok(())
}
