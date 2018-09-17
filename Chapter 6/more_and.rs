// more_and.rs -- using continue and break

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
use std::string::String;

const QUALITY: [&'static str; 4] = [    // an array of &str
    "10,000-meter race.\n",
    "mud tug-of-war.\n",
    "masters canoe jousting.\n",
    "pie-throwing festival.\n",
];

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut age: i32 = 0;
    let _: usize = stdout.write(b"Enter your age in years: ")?;
    let _: () = stdout.flush()?;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    age = input.trim().parse::<i32>().unwrap_or(i32::default());
    let index: usize;

    if age > 17 && age < 35 {
        index = 0;
    } else if age >= 35 && age < 50 {
        index = 1;
    } else if age >= 50 && age < 65 {
        index = 2;
    } else {
        index = 3;
    }

    write!(stdout, "You quality for the {}", QUALITY[index]);

    Ok(())
}
