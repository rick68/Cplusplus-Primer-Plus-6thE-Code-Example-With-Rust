// arrfun2.rs -- functions with an array argument

use std::io;
use std::io::prelude::*;
use std::mem::size_of_val;

const ARSIZE: usize = 8;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let cookies: [i32; ARSIZE] = [1, 2, 4, 8, 16, 32, 64, 128];
    // some systems require preceding i32 with static to
    // enable array initialization

    write!(stdout, "{:p} = array address, ", &cookies);

    write!(stdout, "{} = sizeof cookies\n", size_of_val(&cookies));
    let mut sum: i32 = sum_arr(&cookies, ARSIZE);
    write!(stdout, "Total cookies eaten: {}\n", sum);
    sum = sum_arr(&cookies, 3);         // a lie
    write!(stdout, "First three eaters ate {} cookies.\n", sum);
    sum = sum_arr(&cookies[4..], 4);    // another lie
    write!(stdout, "Last four eaters ate {} cookies.\n", sum);

    Ok(())
}

// return the sum of an integer array
fn sum_arr(arr: &[i32], n: usize) -> i32 {
    let mut stdout: io::Stdout = io::stdout();
    let mut total: i32 = 0;
    write!(stdout, "{:p} = arr, ", arr);

    write!(stdout, "{} = size_of_val(&arr)\n", size_of_val(&arr));
    for i in 0..n {
        total += arr[i];
    }

    total
}
