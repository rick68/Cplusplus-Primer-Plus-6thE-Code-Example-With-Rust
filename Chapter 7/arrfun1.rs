// arrfun1.rs -- functions with an array argument

use std::io;
use std::io::prelude::*;

const ARSIZE: usize = 8;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let cookies: [i32; ARSIZE] = [1, 2, 4, 8, 16, 32, 64, 128];
    // some systems require preceding i32 with static to
    // enable array initialization

    let sum: i32 = sum_arr(&cookies, ARSIZE);
    write!(stdout, "Total cookies eaten: {}\n", sum);

    Ok(())
}

// return the sum of an integer array
fn sum_arr(arr: &[i32], n: usize) -> i32 {
    let mut total: i32 = 0;

    for i in 0..n {
        total += arr[i];
    }

    total
}
