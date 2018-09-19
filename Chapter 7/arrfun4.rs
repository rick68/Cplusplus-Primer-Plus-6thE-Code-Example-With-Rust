// arrfun4.rs -- functions with an array range

use std::io;
use std::io::prelude::*;

const ARSIZE: usize = 8;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let cookies: [i32; ARSIZE] = [1, 2, 4, 8, 16, 32, 64, 128];
    // some systems require preceding i32 with static to
    // enable array initialization

    let mut sum: i32 = sum_arr(&cookies[0..ARSIZE]);
    write!(stdout, "Total cookies eaten: {}\n", sum);
    sum = sum_arr(&cookies[0..3]); // first 3 elements
    write!(stdout, "First three eaters ate {} cookies.\n", sum);
    sum = sum_arr(&cookies[4..8]); // first 3 elements
    write!(stdout, "Last four eaters ate {} cookies.\n", sum);

    Ok(())
}

// return the sum of an integer array
fn sum_arr(arr: &[i32]) -> i32 {
    let mut total: i32 = 0;

    for i in arr {
        total += i;
    }

    total
}
