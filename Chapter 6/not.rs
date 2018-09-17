// not.rs -- using the not operator

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::i32;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut num: f64;

    let _: usize = stdout.write(b"Yo, dude! Enter an integer value: ")?;
    let _: () = stdout.flush()?;
    let mut input: String = String::new();
    loop {
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        num = match input.trim().parse::<f64>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        break;
    }
    while !is_int(num) {        // continue while num is not int-able
        let _: usize = stdout.write(b"Out of range -- please try again: ")?;
        let _: () = stdout.flush()?;
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        num = input.trim().parse::<f64>().unwrap_or(f64::default());
    }
    let val: i32 = num as i32;  // type cast
    write!(stdout, "You've entered the integer {}\nBye\n", val);

    Ok(())
}

fn is_int(x: f64) -> bool {
    if x <= i32::MAX as f64 && x >= i32::MIN as f64 {   // use climits values
        true
    } else {
        false
    }
}
