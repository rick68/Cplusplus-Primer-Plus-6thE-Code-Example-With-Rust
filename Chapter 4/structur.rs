// structur.rs -- a simple structure

use std::io;
use std::io::prelude::*;

struct Inflatable {
    name: &'static str,
    #[allow(unused)]
    volume: f32,
    price: f64,
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let guest: Inflatable = Inflatable {
        name: "Glorious Gloria",
        volume: 1.88,
        price: 29.99,
    }; // guest is a structure variable of type Inflatable

    // It's initialized to the indicated values
    let pal: Inflatable = Inflatable {
        name: "Audacious Arthur",
        volume: 3.12,
        price: 32.99,
    }; // pal is a second variable of type Inflatable

    write!(stdout, "Expand your guest list with {}", guest.name);
    write!(stdout, " and {}!\n", pal.name);
    // pal.name is the name member of the pal variable
    stdout.write(b"You can have both for $")?;
    write!(stdout, "{}!\n", guest.price + pal.price);

    Ok(())
}
