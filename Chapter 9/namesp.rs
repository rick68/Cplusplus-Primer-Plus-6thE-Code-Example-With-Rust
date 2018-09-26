// namesp.rs -- namespaces

use std::io;

pub mod pers {
    use super::io;
    use super::io::prelude::*;

    pub struct Person {
        pub fname: String,
        pub lname: String,
    }

    impl Person {
        pub fn new() -> Person {
            Person {
                fname: String::new(),
                lname: String::new(),
            }
        }
    }

    pub fn get_person(rp: &mut Person) {
        let mut stdout: io::Stdout = io::stdout();
        let stdin: io::Stdin = io::stdin();

        let _: usize = stdout.write(b"Enter first name: ").unwrap();
        stdout.flush().unwrap();
        let mut input: String = String::new();
        let _: usize = stdin.read_line(&mut input).unwrap();
        rp.fname = input.trim().to_string();
        let _: usize = stdout.write(b"Enter last name: ").unwrap();
        stdout.flush().unwrap();
        input.clear();
        let _: usize = stdin.read_line(&mut input).unwrap();
        rp.lname = input.trim().to_string();
    }

    pub fn show_person(rp: &Person) {
        let mut stdout: io::Stdout = io::stdout();
        write!(stdout, "{}, {}", rp.fname, rp.lname);
    }
}

pub mod debts {
    use super::io;
    use super::io::prelude::*;
    use super::pers::Person;

    pub struct Debt {
        pub name: Person,
        pub amount: f64,
    }

    impl Debt {
        pub fn new() -> Debt {
            Debt {
                name: Person::new(),
                amount: f64::default(),
            }
        }
    }

    pub fn get_debt(rd: &mut Debt) {
        let mut stdout: io::Stdout = io::stdout();
        let stdin: io::Stdin = io::stdin();

        super::pers::get_person(&mut rd.name);
        let _: usize = stdout.write(b"Enter debt: ").unwrap();
        stdout.flush().unwrap();
        let mut input: String = String::new();
        let _: usize = stdin.read_line(&mut input).unwrap();
        rd.amount = input.trim().parse::<f64>().unwrap_or(f64::default());
    }

    pub fn show_debt(rd: &Debt) {
        let mut stdout: io::Stdout = io::stdout();

        super::pers::show_person(&rd.name);
        write!(stdout, ": ${}\n", rd.amount);
    }

    pub fn sum_debts(ar: &[Debt], n: usize) -> f64 {
        let mut total: f64 = 0.0;
        for i in 0..n {
            total += ar[i].amount;
        }
        total
    }
}
