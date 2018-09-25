// strtref.rs -- using structure references

use std::io;
use std::io::prelude::*;
use std::clone::Clone;
use std::str;

#[derive(Clone, Copy)]
struct Sysop {
    name: &'static str,
    quote: &'static str,
    used: i32,
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut looper: Sysop = Sysop {
        name: "Rick \"Fortran\" Looper",
        quote: "I'm a goto kind of guy.",
        used: 0,
    };

    let _: &Sysop = to_use(&mut looper);        // looper is type sysop
    write!(stdout, "Lopper: {} use(s)\n", looper.used);
    let copycat: Sysop;
    copycat = to_use(&mut looper).clone();
    write!(stdout, "Looper: {} use(s)\n", looper.used);
    write!(stdout, "Copycat: {} use(s)\n", copycat.used);
    write!(stdout, "to_use(&mut looper): {} use(s)\n", to_use(&mut looper).used);

    Ok(())
}

// to_use() returns the reference passed to it
fn to_use(sysopref: &mut Sysop) -> &Sysop {
    let mut stdout: io::Stdout = io::stdout();

    write!(stdout, "{} says:\n", sysopref.name);
    write!(stdout, "{}\n", sysopref.quote);
    sysopref.used += 1;
    sysopref
}
