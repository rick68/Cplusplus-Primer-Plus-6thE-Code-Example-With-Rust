// arith.rs -- some Rust arithmetic

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let hats: f32;
    let heads: f32;

    stdout.write(b"Enter a number: ")?;
    stdout.flush();
    let mut input: String = String::new();
    stdin.read_line(&mut input)?;
    hats = input.trim().parse::<f32>().unwrap_or(f32::default());
    stdout.write(b"Enter another number: ")?;
    stdout.flush();
    input.clear();
    stdin.read_line(&mut input)?;
    heads = input.trim().parse::<f32>().unwrap_or(f32::default());

    write!(stdout, "hats = {:.*}; heads = {:.*}\n", 6, hats, 6, heads);
    write!(stdout, "hats + heads = {:.*}\n", 6, hats + heads);
    write!(stdout, "hats - heads = {:.*}\n", 6, hats - heads);
    write!(stdout, "hats * heads = {:.*}\n", 6, hats * heads);
    write!(stdout, "hats / heads = {:.*}\n", 6, hats / heads);

    Ok(())
}
