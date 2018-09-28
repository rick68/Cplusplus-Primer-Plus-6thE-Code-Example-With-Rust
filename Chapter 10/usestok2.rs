// usestok2.rs -- using the Stock struct

mod stock20;
use stock20::*;

use std::io;
use std::io::prelude::*;

const STKS: usize = 4;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let stocks: [Stock; STKS] = [
        Stock::new("NanoSmart", 12, 20.0),
        Stock::new("Boffo Objects", 200, 2.0),
        Stock::new("Monolithic Obelisks", 130, 3.25),
        Stock::new("Fleep Enterprises", 60, 6.5),
    ];

    write!(stdout, "Stock holdings:\n");
    for st in 0..STKS {
        stocks[st].show();
    }
    // set pointer to first element
    let mut top: *const Stock = stocks.as_ptr();
    for st in 1..STKS {
        unsafe {
            top = (*top).topval(&stocks[st]) as *const Stock;
        }
    }
    // now top points to the most valuable holding
    write!(stdout, "\nMost valuable holding:\n");
    unsafe {
        (*top).show();
    }

    Ok(())
}
