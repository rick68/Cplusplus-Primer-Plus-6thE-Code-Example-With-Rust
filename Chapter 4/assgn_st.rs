// assgn_st.rs -- assigning structures

use std::io;
use std::io::prelude::*;

struct inflatable {
    name: &'static str,
    volume: f32,
    price: f64,
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let bouquet: inflatable = inflatable {
        name: "sunflowers",
        volume: 0.20,
        price: 12.49,
    };
    let choice: inflatable;

    write!(stdout, "bouquet: {} for $", bouquet.name);
    write!(stdout, "{}\n", bouquet.price);

    choice = bouquet;   // move one structre to another
    write!(stdout, "choice: {} for $", choice.name);
    write!(stdout, "{}\n", choice.price);

    Ok(())
}
