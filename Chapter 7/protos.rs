// protos.rs -- using prototypes and function calls

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    cheers(5);  // function call
    let side: f64;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    side = input.trim().parse::<f64>().unwrap_or(f64::default());
    let volume: f64 = cube(side);// function call
    write!(stdout, "A {} foot cube has a volume of ", side);
    write!(stdout, "{} cubic feet.\n", volume);
    cheers(cube(2 as f64) as usize);     // prototype protection at work

    Ok(())
}

fn cheers(n: usize) {
    let mut stdout: io::Stdout = io::stdout();

    for _ in 0..n {
        let _: usize = stdout.write(b"Cheers! ").unwrap();
    }
    let _: usize = stdout.write(b"\n").unwrap();
}

fn cube(x: f64) -> f64 {
    x * x
}