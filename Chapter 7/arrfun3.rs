// arrfun3.rs -- array functions and const

use std::io;
use std::io::prelude::*;
use std::string::String;

const MAX: usize = 5;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut properties: [f64; MAX] = [0.0; MAX];

    let size: usize = fill_array(&mut properties, MAX);
    show_array(&properties, size);
    if size > 0 {
        let _: usize = stdout.write(b"Enter revaluation factor: ")?;
        stdout.flush()?;
        let factor: f64;
        let mut input: String = String::new();
        loop {
            input.clear();
            match stdin.read_line(&mut input) {
                Err(_) => {
                    let _: usize = stdout.write(b"Bad input; Please enter a number.\n")?;
                    stdout.flush()?;
                    continue;
                }
                _ => (),
            };
            factor = match input.trim().parse::<f64>() {
                Ok(val) => val,
                _ => {
                    continue;
                }
            };
            break;
        }
        revalue(factor, &mut properties, size);
        show_array(&properties, size);
    }
    let _: usize = stdout.write(b"Done.\n")?;

    Ok(())
}

fn fill_array(ar: &mut [f64], limit: usize) -> usize {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut temp: f64;
    let mut input: String = String::new();
    for i in 0..limit {
        write!(stdout, "Enter value #{}: ", i + 1);
        stdout.flush().unwrap();
        input.clear();
        match stdin.read_line(&mut input) {
            Err(_) => {
                let _: usize = stdout
                    .write(b"Bad input; input process terminated.\n")
                    .unwrap();
                stdout.flush().unwrap();
                return i;
            }
            _ => (),
        };
        temp = match input.trim().parse::<f64>() {
            Ok(val) => {
                if val < 0.0 {
                    return i;
                }
                val
            }
            _ => {
                return i;
            }
        };
        ar[i] = temp;
    }
    limit
}

// the following function can use, but not alter,
// the array whose address is ar
fn show_array(ar: &[f64], n: usize) {
    let mut stdout: io::Stdout = io::stdout();

    for i in 0..n {
        write!(stdout, "Property #{}: $", i + 1);
        write!(stdout, " {}\n", ar[i]);
    }
}

// multiplies each element of ar[] by r
fn revalue(r: f64, ar: &mut [f64], n: usize) {
    for i in 0..n {
        ar[i] *= r;
    }
}
