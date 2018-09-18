// sumafile.rs -- functions with an array argument

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut filename: String = String::new();
    let mut in_file: File; // object for handling file input

    let _: usize = stdout.write(b"Enter name of data file: ")?;
    let _: () = stdout.flush()?;
    let _: usize = stdin.read_line(&mut filename)?;
    in_file = match File::open(&filename.trim()) { // associate in_file with a file
        Ok(f) => f,
        Err(e) => {
            write!(stdout, "Could not open the file {}\n", filename.trim());
            let _: usize = stdout.write(b"Program terminating.\n")?;
            return Err(e);
        },
    };
    let mut value: f64;
    let mut sum: f64 = 0.0;
    let mut count: i32 = 0;     // number of items read

    let mut input: String = String::new();
    match in_file.read_to_string(&mut input) {
        Ok(0) => {
            let _: usize = stdout.write(b"End of file reached.\n")?;
            let _: () = stdout.flush()?;
        }
        Err(_) => {
            let _: usize = stdout.write(b"Input terminated by data mismatch.\n")?;
        }
        _ => (),
    };
    'outer: for line in input.lines() {
        for token in line.split(|c| c == ' ' || c == '\t') {
            if token.len() == 0 { continue; }
            value = match token.parse::<f64>() {
                Ok(val) => val,
                Err(_) => {
                    let _: usize = stdout.write(b"Input terminated by data mismatch.\n")?;
                    let _: () = stdout.flush()?;
                    break 'outer;
                },
            };
            count += 1;
            sum += value;
        }
    };
    if count == 0 {
        let _: usize = stdout.write(b"No data processed.\n")?;
    } else {
        write!(stdout, "Items read: {}\n", count);
        write!(stdout, "Sum: {}\n", sum);
        write!(stdout, "Average: {}\n", sum / count as f64);
    }

    Ok(())
}
