// init_ptr.rs -- initialize a pointer

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let higgens: i32 = 5;
    let pt: *const i32 = &higgens;

    write!(
        stdout,
        "value of higgens = {}; Address of higgens = {:p}\n",
        higgens, &higgens
    );
    write!(
        stdout,
        "value of *pt = {}; Value of pt = {:p}\n",
        unsafe { *pt },
        pt
    );

    Ok(())
}
