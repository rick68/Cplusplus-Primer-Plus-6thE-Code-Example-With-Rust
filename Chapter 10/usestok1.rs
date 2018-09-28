// usestok1.rs -- using the Stock struct

mod stock10;
use stock10::*;

use std::io;
use std::io::prelude::*;
use std::mem;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stock1: Stock = Stock::new("NanoSmart", 12, 20.0);   // syntax 1
    stock1.show();
    let mut stock2: Stock = Stock::new("Boffo Objects", 2, 2.0); // syntax 2
    stock2.show();

    write!(stdout, "Assigning stock1 to stack2:\n");
    stock2 = stock1.clone();
    write!(stdout, "Listing stock1 to stack2:\n");
    stock1.show();
    stock2.show();

    write!(stdout, "Using a constructor to reset an object\n");
    mem::forget(stock1);        // forget the stock1 so its destructor doesn't run.
    stock1 = Stock::new("Nifty Foods", 10, 50.0);       // temp object
    write!(stdout, "Revised stock1:\n");
    stock1.show();
    write!(stdout, "Done\n");

    Ok(())
}
