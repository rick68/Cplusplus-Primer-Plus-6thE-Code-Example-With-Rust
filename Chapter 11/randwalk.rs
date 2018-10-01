// rnadwalk.rs -- using the Vector struct

#![feature(libc)]
extern crate libc;

#[macro_use]
mod vect;
use vect::*;

use std::io;
use std::io::prelude::*;
use std::ptr;
use std::string::String;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    unsafe {
        libc::srand(libc::time(ptr::null_mut()) as libc::c_uint);       // seed random-number generator
    }

    let mut direction: f64;
    let mut step: Vector = Vector_new!();
    let mut result: Vector = Vector_new!(0.0, 0.0);
    let mut steps: usize = 0;
    let mut target: f64;
    let mut dstep: f64;

    let mut ch: [u8; 1] = <[u8; 1]>::default();
    let mut token: String = String::new();
    'outer: loop {
        let _: usize = stdout.write(b"Enter target distance (q to quit): ")?;
        stdout.flush()?;
        token.clear();
        loop {
            match stdin.read(&mut ch) {
                Ok(0) => break 'outer, // EOF
                Ok(_) => {
                    let c: char = ch[0] as char;
                    if token.is_empty() && c == 'q' {
                        break 'outer;
                    }
                    else if c != ' ' && c != '\n' {
                        token.push(c);
                        continue;
                    } else if !token.is_empty() {
                        break;
                    }
                }
                Err(_) => break 'outer,
            }
        }
        target = token.parse().unwrap_or(f64::default());
        let _: usize = stdout.write(b"Enter step length: ")?;
        stdout.flush()?;
        token.clear();
        loop {
            match stdin.read(&mut ch) {
                Ok(0) => break 'outer, // EOF
                Ok(_) => {
                    let c: char = ch[0] as char;
                    if c != ' ' && c != '\n' {
                        token.push(c);
                        continue;
                    } else if !token.is_empty() {
                        break;
                    }
                }
                Err(_) => break 'outer,
            }
        }
        dstep = token.parse().unwrap_or(f64::default());

        while result.magval() < target {
            direction = (unsafe { libc::rand() } % 360).into();
            Vector_reset!(step, dstep, direction, Mode::POL);
            result = result + step.clone();
            steps += 1;
        }
        write!(stdout, "After {} steps, the subject", steps);
        let _: usize = stdout.write(b"has the following location:\n")?;
        write!(stdout, "{}\n", result);
        result.polar_mode();
        write!(stdout, " or\n{}\n", result);
        write!(stdout, "Average outward distance per step = {}\n", result.magval() / (steps as f64));
        steps = 0;
        Vector_reset!(result, 0.0, 0.0);
    }
    let _: usize = stdout.write(b"Bye!\n")?;

    Ok(())
}
