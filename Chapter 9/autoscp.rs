// autoscp.rs -- illustrating scope of automatic variables

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let texas: i32 = 31;
    let year: i32 = 2011;

    write!(stdout, "In main(), texas = {}, &texas = ", texas);
    write!(stdout, "{:p}\n", &texas);
    write!(stdout, "In main(), year = {}, &year = ", year);
    write!(stdout, "{:p}\n", &year);
    oil(texas);
    write!(stdout, "In main(), texas = {}, &texas = ", texas);
    write!(stdout, "{:p}\n", &texas);
    write!(stdout, "In main(), year = {}, &year = ", year);
    write!(stdout, "{:p}\n", &year);

    Ok(())
}

fn oil(x: i32) {
    let mut stdout: io::Stdout = io::stdout();
    let texas: i32 = 5;

    write!(stdout, "In oil(), texas = {}, &texas = ", texas);
    write!(stdout, "{:p}\n", &texas);
    write!(stdout, "In oil(), x = {}, &x = ", x);
    write!(stdout, "{:p}\n", &x);
    {   // start a block
        let texas: i32 = 113;
        write!(stdout, "In block, texas = {}", texas);
        write!(stdout, ", &texas = {:p}\n", &texas);
        write!(stdout, "In block, x = {}", x);
        write!(stdout, ", &x = {:p}\n", &x);
    }   // end a block
    write!(stdout, "Post-block texas = {}", texas);
    write!(stdout, ", &texas = {:p}\n", &texas);
}
