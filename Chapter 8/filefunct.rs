// filefunct.rs -- function with Stdout & parameter

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;
use std::string::String;

const LIMIT: usize = 5;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let mut stdin: io::Stdin = io::stdin();

    let mut fout: File;
    let fname: &str = "ep-data.txt";
    fout = match File::create(fname) {
        Ok(f) => f,
        Err(_) => {
            write!(stdout, "Can't open {}. Bye.\n", fname);
            process::exit(1);
        }
    };
    let objective: f64;
    let _: usize = stdout.write(b"Enter the focal length of your ")?;
    let _: usize = stdout.write(b"telescope objective in mm: ")?;
    stdout.flush()?;
    let mut input: [u8; 1] = <[u8; 1]>::default();
    let mut token: String = String::new();
    loop {
        token.clear();
        loop {
            match stdin.read(&mut input) {
                Ok(0) => (), // EOF
                Ok(_) => {
                    if input[0] != ' ' as u8 && input[0] != '\n' as u8 {
                        token.push(input[0] as char);
                        continue;
                    }
                }
                Err(_) => (),
            };
            break;
        }
        objective = match token.parse::<f64>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        break;
    }
    let mut eps: [f64; LIMIT] = <[f64; LIMIT]>::default();
    write!(stdout, "Enter the fcal lengths, in mm, of {} ", LIMIT);
    let _: usize = stdout.write(b" eyepieces:\n")?;
    for i in 0..LIMIT {
        write!(stdout, "Eyepiece #{}: ", i + 1);
        stdout.flush()?;
        token.clear();
        loop {
            token.clear();
            loop {
                match stdin.read(&mut input) {
                    Ok(0) => (), // EOF
                    Ok(_) => {
                        if input[0] != ' ' as u8 && input[0] != '\n' as u8 {
                            token.push(input[0] as char);
                            continue;
                        }
                    }
                    Err(_) => (),
                };
                break;
            }
            eps[i] = match token.parse::<f64>() {
                Ok(val) => val,
                Err(_) => continue,
            };
            break;
        }
    }
    file_it(&mut fout, objective, &eps, LIMIT);
    file_it(&mut stdout, objective, &eps, LIMIT);

    Ok(())
}

fn file_it(os: &mut Write, fo: f64, fe: &[f64], n: usize) {
    write!(os, "Focal length of objective: {} mm\n", fo);
    write!(os, "{:12}", "f.l. eyepiece");
    write!(os, "{:>16}", "magnification\n");
    for i in 0..n {
        write!(os, "{:12.1}", fe[i]);
        write!(os, "{:15}\n", (fo / fe[i] + 0.5) as i32);
    }
}
