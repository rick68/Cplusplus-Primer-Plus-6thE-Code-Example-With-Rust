// add.rs -- using the logical AND operator

use std::io;
use std::io::prelude::*;
use std::string::String;

const ARSIZE: usize = 6;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut naaq: [f32; ARSIZE] = <[f32; ARSIZE]>::default();

    let _: usize = stdout.write(b"Enter the NAAQs (New Age Awareness Quotients) ")?;
    let _: usize = stdout.write(b"of\nyour neighbors. Program terminates ")?;
    write!(stdout, "when you make\n{} entries ", ARSIZE);
    let _: usize = stdout.write(b"or enter a negative value.\n")?;
    let mut i: usize = 0;
    let mut temp: f32;
    let _: usize = stdout.write(b"First value: ")?;
    let _: () = stdout.flush()?;
    let mut input: String = String::new();
    let _: usize = stdin.read_line(&mut input)?;
    temp = input.trim().parse::<f32>().unwrap_or(f32::default());
    while i < ARSIZE && temp >= 0.0 {           // 2 quitting criteria
        naaq[i] = temp;
        i += 1;
        if i < ARSIZE {                         // room left in the array
            write!(stdout, "Next value: ");
            let _: () = stdout.flush()?;
            input.clear();
            let _: usize = stdin.read_line(&mut input)?;
            temp = input.trim().parse::<f32>().unwrap_or(f32::default()); // so get next value
        }
    }
    if i == 0 {
        let _: usize = stdout.write(b"No data--bye\n")?;
    } else {
        let _: usize = stdout.write(b"Enter your NAAQ: ")?;
        let _: () = stdout.flush()?;
        let you: f32;
        input.clear();
        let _: usize = stdin.read_line(&mut input)?;
        you = input.trim().parse::<f32>().unwrap_or(f32::default());
        let mut count: i32 = 0;
        for j in 0..i {
            if naaq[j] > you {
                count += 1;
            }
        }
        write!(stdout, "{}", count);
        let _: usize = stdout.write(b" of your neighbors have greater awareness of\n")?;
        let _: usize = stdout.write(b"the New Age than you do.\n")?;
    }

    Ok(())
}
