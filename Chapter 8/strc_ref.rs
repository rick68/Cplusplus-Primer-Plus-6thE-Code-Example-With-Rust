// strc_ref.rs -- using structure references

#![allow(unused_assignments)]

use std::io;
use std::io::prelude::*;
use std::default::Default;
use std::clone::Clone;

#[derive(Default, Clone, Copy)]
struct FreeThrows {
    name: &'static str,
    made: i32,
    attempts: i32,
    percent: f32,
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut one: FreeThrows = FreeThrows {
        name: "Ifelsa Branch",
        made: 13,
        attempts: 14,
        percent: 0.0,
    };
    let two: FreeThrows = FreeThrows {
        name: "Andor Knott",
        made: 10,
        attempts: 16,
        percent: 0.0,
    };
    let three: FreeThrows = FreeThrows {
        name: "Minnie Max",
        made: 7,
        attempts: 9,
        percent: 0.0,
    };
    let mut four: FreeThrows = FreeThrows {
        name: "Whily Looper",
        made: 5,
        attempts: 9,
        percent: 0.0,
    };
    let five: FreeThrows = FreeThrows {
        name: "Long Long",
        made: 6,
        attempts: 14,
        percent: 0.0,
    };
    let mut team: FreeThrows = FreeThrows {
        name: "Throwgoods",
        made: 0,
        attempts: 0,
        percent: 0.0,
    };
    let mut dup: FreeThrows = FreeThrows::default();

    set_pc(&mut one);
    display(&one);
    accumulate(&mut team, &one);
    display(&team);
    // use return value as argument
    display(accumulate(&mut team, &two));
    accumulate(accumulate(&mut team, &three), &four);
    display(&team);
    // use return value in assignment
    dup = accumulate(&mut team, &five).clone();
    stdout.write(b"Displaying team:\n")?;
    display(&team);
    stdout.write(b"Displaying dup after assignment:\n")?;
    display(&dup);
    set_pc(&mut four);
    // ill-advised assignment
    *accumulate(&mut dup, &five) = four;
    stdout.write(b"Displaying dup after ill-advised asignment:\n")?;
    display(&dup);

    Ok(())
}

fn display(ft: &FreeThrows) {
    let mut stdout: io::Stdout = io::stdout();
    write!(stdout, "Name: {}\n", ft.name);
    write!(stdout, "  Made: {}\t", ft.made);
    write!(stdout, "Attempts: {}\t", ft.attempts);
    write!(stdout, "Percent: {}\n", ft.percent);
}

fn set_pc(ft: &mut FreeThrows) {
    if ft.attempts != 0 {
        ft.percent = 100.0 * ft.made as f32 / ft.attempts as f32;
    } else {
        ft.percent = 0.0;
    }
}

fn accumulate<'a>(target: &'a mut FreeThrows, source: &FreeThrows) -> &'a mut FreeThrows {
    target.attempts += source.attempts;
    target.made += source.made;
    set_pc(target);
    target
}
