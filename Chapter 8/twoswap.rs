// twoswap.rs -- specialization overrides a template

use std::io;
use std::io::prelude::*;
use std::str;
use std::clone::Clone;
use std::marker::Copy;

struct Job {
    name: &'static str,
    salary: f64,
    floor: i32,
}

macro_rules! swap {
    ($a:expr; &mut T, $b:expr; &mut T) => (
        swap($a, $b)
    );
    ($j1:expr; &mut Job, $j2:expr; &mut Job) => (
        swap_job($j1, $j2)
    );
}

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let mut i: i32 = 10;
    let mut j: i32 = 20;
    write!(stdout, "i, j = {}, {}.\n", i, j);
    let _: usize = stdout.write(b"Using compiler-generated i32 swapper:\n")?;
    swap!(&mut i; &mut T, &mut j; &mut T);
    write!(stdout, "i, j = {}, {}.\n", i, j);

    let mut sue: Job = Job {
        name: "Susan Yaffee",
        salary: 73000.60,
        floor: 7,
    };
    let mut sidney: Job = Job {
        name: "Sidney Taffee",
        salary: 78060.72,
        floor: 9,
    };
    let _: usize = stdout.write(b"Before job swapping:\n")?;
    show(&sue);
    show(&sidney);
    swap!(&mut sue; &mut Job, &mut sidney; &mut Job);
    let _: usize = stdout.write(b"After job swapping:\n")?;
    show(&sue);
    show(&sidney);

    Ok(())
}

fn swap<T: Clone + Copy>(a: &mut T, b: &mut T) { // general version
    let temp: T;
    temp = *a;
    *a = *b;
    *b = temp;
}

// swaps just the salary and floor fields of a job structure

fn swap_job(j1: &mut Job, j2: &mut Job) { // specialization
    let t1: f64;
    let t2: i32;
    t1 = j1.salary;
    j1.salary = j2.salary;
    j2.salary = t1;
    t2 = j1.floor;
    j1.floor = j2.floor;
    j2.floor = t2;
}

fn show(j: &Job) {
    let mut stdout: io::Stdout = io::stdout();
    write!(stdout, "{}:$ {} on floor {}\n", j.name, j.salary, j.floor);
}
