// file1.rs -- example of a three-file program

mod coordin;
mod file2;

use std::io;
use std::io::prelude::*;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut rplace: coordin::Rect = coordin::Rect::default();
    let mut pplace: coordin::Polar;

    let _: usize = stdout.write(b"Enter the x and y values: ")?;
    stdout.flush()?;
    let mut input: [u8; 1] = <[u8; 1]>::default();
    let mut token: String = String::new();
    'outer: loop {      // slick use of stdin
        token.clear();
        loop {
            match stdin.read(&mut input) {
                Ok(0) => (), // EOF
                Ok(_) => {
                    if input[0] != ' ' as u8 && input[0] != '\n' as u8 {
                        token.push(input[0] as char);
                        continue;
                    } else if token.len() == 0 {
                        continue;
                    }
                }
                Err(_) => (),
            }
            rplace.x = match token.parse::<f64>() {
                Ok(val) => val,
                Err(_) => break 'outer,
            };
            break;
        }
        token.clear();
        loop {
            match stdin.read(&mut input) {
                Ok(0) => (), // EOF
                Ok(_) => {
                    if input[0] != ' ' as u8 && input[0] != '\n' as u8 {
                        token.push(input[0] as char);
                        continue;
                    } else if token.len() == 0 {
                        continue;
                    }
                }
                Err(_) => (),
            }
            rplace.y = match token.parse::<f64>() {
                Ok(val) => val,
                Err(_) => break 'outer,
            };
            break;
        }
        pplace = file2::rect_to_polar(&rplace);
        file2::show_polar(&pplace);
        let _: usize = stdout.write(b"Next two numbers (q to quit): ")?;
        stdout.flush()?;
    }
    let _: usize = stdout.write(b"Bye!\n")?;

    Ok(())
}
