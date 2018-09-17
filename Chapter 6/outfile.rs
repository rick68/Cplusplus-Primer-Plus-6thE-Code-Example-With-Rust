// outfile.rs -- writing to a file

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut automobile: String = String::new();
    let year: i32;
    let a_price: f64;
    let d_price: f64;

    let _: usize = stdout.write(b"Enter the make and model of automobile: ")?;
    let _: () = stdout.flush()?;
    let _: usize = stdin.read_line(&mut automobile)?;
    let _: usize = stdout.write(b"Enter the model year: ")?;
    let _: () = stdout.flush()?;
    let mut input: String = String::new();
    loop {
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        year = match input.trim().parse::<i32>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        break;
    }
    let _: usize = stdout.write(b"Enter the original asking price: ")?;
    let _: () = stdout.flush()?;
    loop {
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        a_price = match input.trim().parse::<f64>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        break;
    }
    d_price = 0.913 * a_price;

    // display information on screen with stdout

    write!(stdout, "Make and model: {}\n", automobile.trim());
    write!(stdout, "Year: {}\n", year);
    write!(stdout, "Was asking ${:.*}\n", 2, a_price);
    write!(stdout, "Now asking ${:.*}\n", 2, d_price);

    // now do exact same things using outFile instead of stdout

    {
        let mut out_file: File = File::create("carinfo.txt")?; // create object for output and associate with a file
        write!(out_file, "Make and model: {}\n", automobile.trim());
        write!(out_file, "Year: {}\n", year);
        write!(out_file, "Was asking ${:.*}\n", 2, a_price);
        write!(out_file, "Now asking ${:.*}\n", 2, d_price);
    } // done with file

    Ok(())
}
