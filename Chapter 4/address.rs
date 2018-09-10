// address.rs -- using the & operator to find addresses

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let donuts: i32 = 6;
    let cups: f64 = 4.5;

    write!(stdout, "donuts value = {}", donuts);
    write!(stdout, " and donuts address = {:p}\n", &donuts);

    // Note: you may need to use unsigned (&donuts)
    // and unsigned (&cups)

    write!(stdout, "cups value = {}", cups);
    write!(stdout, " and cups address = {:p}\n", &cups);

    Ok(())
}
