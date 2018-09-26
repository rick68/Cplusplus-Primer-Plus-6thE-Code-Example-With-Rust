// twofile2.rs -- variables with internal and external linkage

use std::io;
use std::io::prelude::*;

use ::TOM;                      // TOM defined elsewhere
static DICK:i32 = 10;
static HARRY: i32 = 200;

pub fn remote_access() {
    let mut stdout: io::Stdout = io::stdout();

    let _: usize = stdout.write(b"remote_access() reports the following addresses:\n").unwrap();
    write!(stdout, "{:p} = &TOM, {:p} = &DICK, ", &TOM, &DICK);
    write!(stdout, "{:p} = &HARRY\n", &HARRY);
}
