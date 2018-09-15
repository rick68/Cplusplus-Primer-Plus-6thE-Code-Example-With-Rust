// nested.rs -- nested loops and 2-D array

use std::io;
use std::io::prelude::*;

const CITIES: usize = 5;
const YEARS: usize = 4;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let cities: [&'static str; CITIES] = [      // array of str to 5 strings
        "Gribble City",
        "Gribbletown",
        "New Gribble",
        "San Gribble",
        "Gribble Vista",
    ];

    let maxtemps: [[i32; CITIES]; YEARS] = [    // 2-D array
        [96, 100, 87, 101, 105],        // value for maxtemps[0]
        [96, 98, 91, 107, 104],         // value for maxtemps[1]
        [97, 101, 93, 108, 107],        // value for maxtemps[2]
        [98, 103, 95, 109, 108],        // value for maxtemps[3]
    ];

    let _: usize = stdout.write(b"Maximum temperatures for 2008 - 2011\n\n")?;
    for city in 0..CITIES {
        write!(stdout, "{}:\t", cities[city]);
        for year in 0..YEARS {
            write!(stdout, "{}\t", maxtemps[year][city]);
        }
        let _: usize = stdout.write(b"\n")?;
    }

    Ok(())
}
