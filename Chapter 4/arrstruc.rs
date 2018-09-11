// arrstruc.rs -- an array of structures

use std::io;
use std::io::prelude::*;
struct Inflatable {
    name: &'static str,
    volume: f32,
    #[allow(unused)]
    price: f64,
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let guests: [Inflatable; 2] = // initializing an array of structs
        [
            Inflatable {
                name: "Bambi",
                volume: 0.5,
                price: 21.99,
            }, // first structure in array
            Inflatable {
                name: "Godzilla",
                volume: 2000.0,
                price: 565.99,
            }, // next structure in array
        ];

    write!(
        stdout,
        "The guests {} and {}\nhave a combined volume of {} cubic feet.\n ",
        guests[0].name,
        guests[1].name,
        guests[0].volume + guests[1].volume
    );

    Ok(())
}
