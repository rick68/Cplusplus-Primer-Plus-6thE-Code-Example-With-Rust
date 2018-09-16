// waiting.rs -- using std::time::Instant::now() in a time-delay loop

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let _: usize = stdout.write(b"Enter the delay time, in seconds: ")?;
    let _: () = stdout.flush()?;
    let secs: f64;
    let mut input = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    secs = input.trim().parse::<f64>().unwrap_or(f64::default());
    let delay: Duration = Duration::new(secs.trunc() as u64, secs.fract() as u32);
    let _: usize = stdout.write(b"starting\n")?;
    let start: Instant = Instant::now();
    while Instant::now().duration_since(start) < delay {};      // wait until time elapses
    let _: usize = stdout.write(b"down\n")?;

    Ok(())
}
