// stone.rs -- user-defined conversions
// compile with stonewt.rs

#[macro_use]
mod stonewt;
use stonewt::*;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut incognito: Stonewt = Stonewt_new!(275.0); // uses constructor to initialize
    let wolfe: Stonewt = Stonewt_new!(285.7); // same as Stonewt wolfe = 285.7
    let mut taft: Stonewt = Stonewt_new!(21, 8.0);

    let _: usize = stdout.write(b"The celebrity weighed ")?;
    incognito.show_stn();
    let _: usize = stdout.write(b"The detective weighed ")?;
    wolfe.show_stn();
    let _: usize = stdout.write(b"The President weighed ")?;
    taft.show_lbs();
    incognito = Stonewt_new!(276.8); // use constructor for conversion
    taft = Stonewt_new!(325.0);
    let _: usize = stdout.write(b"After dinner, the celebrity weighed ")?;
    incognito.show_stn();
    let _: usize = stdout.write(b"After dinner, the President weighed ")?;
    taft.show_lbs();
    display(&taft, 2);
    let _: usize = stdout.write(b"The wrestler weighted even more.\n")?;
    display(&Stonewt_new!(422.0), 2);
    let _: usize = stdout.write(b"No stone left unearned\n")?;

    Ok(())
}

fn display(st: &Stonewt, n: usize) {
    let mut stdout: io::Stdout = io::stdout();

    for _ in 0..n {
        let _: usize = stdout.write(b"Wow! ").unwrap();
        st.show_stn();
    }
}
