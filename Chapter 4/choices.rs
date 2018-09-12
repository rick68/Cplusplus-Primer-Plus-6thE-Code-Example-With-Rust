// choices.rs -- array variations

use std::io;
use std::io::prelude::*;
use std::vec::Vec;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    // array
    let a1: [f64; 4] = [1.2, 2.4, 3.6, 4.8];
    // vector collection
    let mut a2: Vec<f64> = vec![0.0; 4]; // create vector with 4 elements
    a2[0] = 1.0 / 3.0;
    a2[1] = 1.0 / 5.0;
    a2[2] = 1.0 / 7.0;
    a2[3] = 1.0 / 9.0;
    // use array notation
    write!(stdout, "a1[2]: {} at {:p}\n", a1[2], &a1[2]);
    write!(stdout, "a2[2]: {} at {:p}\n", a2[2], &a2[2]);

    Ok(())
}
