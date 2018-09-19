// strctptr.rs -- functions with pointer to structure arguments

use std::io;
use std::io::prelude::*;
use std::string::String;

struct Polar {
    distance: f64,      // distance from origin
    angle: f64,         // direction from origin
}

struct Rect {
    x: f64,             // horizontal distance from origin
    y: f64,             // vertical distance from origin
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut rplace: Rect = Rect {
        x: 0.0,
        y: 0.0,
    };
    let mut pplace: Polar = Polar {
        distance: 0.0,
        angle: 0.0,
    };

    let _: usize = stdout.write(b"Enter the x and y values: ")?;
    stdout.flush()?;
    let mut token: String = String::new();
    let mut ch: [u8; 1] = [0];
    loop {
        token.clear();
        loop {
            match stdin.read(&mut ch) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    if ch[0] == ' ' as u8 || ch[0] == '\n' as u8 { break; }
                    token.push(ch[0] as char)
                },
                Err(_) => break,
            };
        }
        rplace.x = match token.parse::<f64>() {
            Ok(val) => val,
            Err(_) => break,
        };
        token.clear();
        loop {
            match stdin.read(&mut ch) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    if ch[0] == ' ' as u8 || ch[0] == '\n' as u8 { break; }
                    token.push(ch[0] as char)
                },
                Err(_) => break,
            };
        }
        rplace.y = match token.parse::<f64>() {
            Ok(val) => val,
            Err(_) => break,
        };

        rect_to_polar(&rplace, &mut pplace);   // pass addresses
        show_polar(&pplace);                   // pass address
        let _: usize = stdout.write(b"Next two numbers (q to quit): ")?;
        stdout.flush()?;
    }
    let _: usize = stdout.write(b"Done.\n")?;

    Ok(())
}

// show polar coorinates, converting angle to degrees
fn show_polar(pda: &Polar) {
    let mut stdout: io::Stdout = io::stdout();

    const RAD_TO_DEG: f64 = 57.29577951;

    write!(stdout, "distance = {}", pda.distance);
    write!(stdout, ", angle = {}", pda.angle * RAD_TO_DEG);
    write!(stdout, " degress\n");
}

// convert rectangular to polar coordinates
fn rect_to_polar(pxy: &Rect, pda: &mut Polar) {
    pda.distance = f64::sqrt(pxy.x * pxy.x + pxy.y * pxy.y);
    pda.angle = f64::atan2(pxy.y, pxy.x);
}
