// support.rs -- use external variable
// compile with external.rs

use std::io;
use std::io::prelude::*;

pub static mut WARMING: f64 = 0.3;  // WARMING defined

pub fn update(dt: f64) {    // modifies global variable
    let mut stdout: io::Stdout = io::stdout();
    unsafe {
        WARMING += dt;  // uses global WARMING
    }
    write!(stdout, "Updating global WARMING to {}", unsafe { WARMING });
    let _: usize = stdout.write(b" degrees.\n").unwrap();
}
