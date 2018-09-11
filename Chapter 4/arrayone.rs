// arrayone.rs -- small arrays of integers

use std::io;
use std::io::prelude::*;
use std::mem::size_of_val;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut yams: [i32; 3] = <[i32; 3]>::default(); // creates array with three elements
    yams[0] = 7; // assign value to first element
    yams[1] = 8;
    yams[2] = 6;

    let yamcosts: [i32; 3] = [20, 30, 5]; // create, initialize array

    stdout.write(b"Total yams = ")?;
    write!(stdout, "{}\n", yams[0] + yams[1] + yams[2]);
    write!(stdout, "The package with {} yams costs ", yams[1]);
    write!(stdout, "{} cents per yam.\n", yamcosts[1]);
    let mut total: i32 = yams[0] * yamcosts[0] + yams[1] * yamcosts[1];
    total = total + yams[2] * yamcosts[2];
    write!(stdout, "The total yam expense is {} cents.\n", total);

    write!(stdout, "\nSize of yams array = {}", size_of_val(&yams));
    stdout.write(b" bytes.\n")?;
    write!(stdout, "Size of one element = {}", size_of_val(&yams[0]));
    stdout.write(b" bytes.\n")?;

    Ok(())
}
