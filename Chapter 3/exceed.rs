// exceed.rs -- exceeding some integer limits

use std::io;
use std::io::prelude::*;
use std::i16;
use std::num::Wrapping;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let sam: Wrapping<i16> = Wrapping(i16::MAX);        // initialize a variable to max value
    let sue: Wrapping<u16> = Wrapping(sam.0 as u16);    // okay if variable sam alraedy defined

    write!(stdout, "Sam has {} dollars and Sue has {}", sam , sue);
    stdout.write(b" dollars deposited.\nAdd $1 to each account.\nNow ")?;
    let sam = sam + Wrapping(1i16);
    let sue = sue + Wrapping(1u16);
    write!(stdout, "Sam has {} dollars and Sue has {}", sam , sue);
    stdout.write(b" dollars deposited.\nPoor Sam!\n")?;
    let sam = Wrapping(0i16);
    let sue = Wrapping(0u16);
    write!(stdout, "Sam has {} dollars and Sue has {}", sam , sue);
    stdout.write(b" dollars deposited.\n")?;
    stdout.write(b"Take $1 from each account.\nNow ")?;
    let sam = sam - Wrapping(1i16);
    let sue = sue - Wrapping(1u16);
    write!(stdout, "Sam has {} dollars and Sue has {}", sam , sue);
    stdout.write(b" dollars deposited.\nLucky Sue!\n")?;

    Ok(())
}
