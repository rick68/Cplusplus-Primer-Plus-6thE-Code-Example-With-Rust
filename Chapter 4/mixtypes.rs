// mixtypes.rs -- some type combinations

use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct AntarcticaYearsEnd {
    year: i32,
    /* some really interesting data, etc. */
}

impl AntarcticaYearsEnd {
    fn new() -> AntarcticaYearsEnd {
        AntarcticaYearsEnd {
            year: i32::default(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut s01: AntarcticaYearsEnd = AntarcticaYearsEnd::new();
    s01.year = 1998;
    let mut s02: AntarcticaYearsEnd = AntarcticaYearsEnd::new();
    let pa: *mut AntarcticaYearsEnd = &mut s02;
    unsafe { (*pa).year = 1999 };
    let mut s03: AntarcticaYearsEnd = AntarcticaYearsEnd::new();
    let mut trio: [AntarcticaYearsEnd; 3] = [AntarcticaYearsEnd::new(); 3]; // array of 3 structurse.
    trio[0].year = 2003;
    write!(stdout, "{}\n", trio[0].year);
    let mut arp: [*mut AntarcticaYearsEnd; 3] = [&mut s01, &mut s02, &mut s03];
    write!(stdout, "{}\n", unsafe { (*arp[1]).year });
    let mut ppa: *mut [*mut AntarcticaYearsEnd; 3] = &mut arp;
    write!(stdout, "{}\n", unsafe { (*(*ppa)[0]).year });

    Ok(())
}
