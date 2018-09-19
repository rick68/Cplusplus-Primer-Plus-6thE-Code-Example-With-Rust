// arrfun1.rs -- functions with an array argument

use std::io;
use std::io::prelude::*;
use std::str;

const LEN: usize = 66;
const DIVS: usize = 6;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut ruler: [u8; LEN] = [0; LEN];
    for i in 1..LEN - 2 {
        ruler[i] = ' ' as u8;
    }
    ruler[LEN - 1] = 0;
    let max: usize = LEN - 2;
    let min: usize = 0;
    ruler[min] = '|' as u8;
    ruler[max] = '|' as u8;
    write!(stdout, "{}\n", str::from_utf8(&ruler).unwrap());
    for i in 1..=DIVS {
        subdivide(&mut ruler, min, max, i);
        write!(stdout, "{}\n", str::from_utf8(&ruler).unwrap())?;
        for j in i..(LEN - 2) {
            ruler[j] = ' ' as u8; // reset to blank ruler
        }
    }

    Ok(())
}

fn subdivide(ar: &mut [u8], low: usize, high: usize, level: usize) {
    if level == 0 {
        return;
    }
    let mid: usize = (high + low) / 2;
    ar[mid] = '|' as u8;
    subdivide(ar, low, mid, level - 1);
    subdivide(ar, mid, high, level - 1);
}
