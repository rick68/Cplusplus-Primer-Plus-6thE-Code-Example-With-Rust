// twofile1.rs -- variables with external and internal linkage

mod twofile2;

use std::io;
use std::io::prelude::*;

static TOM: i32 = 3;
static DICK:i32 = 30;
static HARRY: i32 = 300;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let _: usize = stdout.write(b"main() reports the following addresses:\n")?;
    write!(stdout, "{:p} = &TOM, {:p} = &DICK, ", &TOM, &DICK);
    write!(stdout, "{:p} = &HARRY\n", &HARRY);
    twofile2::remote_access();

    Ok(())
}
