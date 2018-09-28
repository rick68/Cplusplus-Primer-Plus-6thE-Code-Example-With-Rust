// usestok0.rs -- the whole program

mod stock00;
use stock00::*;

fn main() {
    let mut stock1: Stock = Stock::new();
    stock1.acquire("NanoSmart", 20, 12.50);
    stock1.show();
    stock1.buy(15, 18.25);
    stock1.show();
    stock1.sell(400, 20.00);
    stock1.show();
}
