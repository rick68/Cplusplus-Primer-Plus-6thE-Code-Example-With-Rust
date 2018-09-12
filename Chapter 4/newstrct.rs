// newstrct.rs -- using new method with a structure

use std::io;
use std::io::prelude::*;
use std::str;
use std::string::String;

struct Inflatable {
    name: [u8; 20],
    volume: f32,
    price: f64,
}

impl Inflatable {
    fn new() -> Inflatable {
        Inflatable {
            name: <[u8; 20]>::default(),
            volume: f32::default(),
            price: f64::default(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let ps: *mut Inflatable = &mut Inflatable::new();
    stdout.write(b"Enter name of inflatable item: ")?;
    stdout.flush()?;
    let input: &mut [u8; 20] = &mut unsafe { &mut (*ps) }.name;
    let n: usize = stdin.read(input)?; // method1 for member access
    input[n - 1] = 0; // remove newline
    stdout.write(b"Enter volume in cubic feet: ")?;
    stdout.flush()?;
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    unsafe { &mut (*ps) }.volume = input.trim().parse::<f32>().unwrap_or(f32::default()); // method2 for member access
    stdout.write(b"Enter price: $")?;
    stdout.flush()?;
    input.clear();
    stdin.read_line(&mut input)?;
    unsafe { &mut (*ps) }.price = input.trim().parse::<f64>().unwrap_or(f64::default());

    write!(
        stdout,
        "Name: {}\n",
        str::from_utf8(&unsafe { &(*ps) }.name).unwrap()
    );
    write!(stdout, "Volume: {} cubic feet\n", unsafe { &(*ps) }.volume);
    write!(stdout, "Price: ${}\n", unsafe { &(*ps) }.price);

    Ok(())
}
