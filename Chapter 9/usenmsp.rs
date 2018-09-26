// usenmsp.rs -- using namespaces

mod namesp;

use std::io;
use std::io::prelude::*;

use namesp::*;

fn main() -> io::Result<()> {
    use pers::Person;
    use debts::{Debt, show_debt};

    let golf: Debt = Debt {
        name: Person {
            fname: "Benny".to_string(),
            lname: "Goatsniff".to_string(),
        },
        amount: 120.0,
    };
    show_debt(&golf);
    other();
    another();

    Ok(())
}

fn other() {
    use pers::{Person, show_person};
    use debts::{Debt, get_debt, show_debt, sum_debts};

    let mut stdout: io::Stdout = io::stdout();

    let dg: Person = Person {
        fname: "Doodles".to_string(),
        lname: "Glister".to_string(),
    };
    show_person(&dg);
    let _: usize = stdout.write(b"\n").unwrap();
    let mut zippy: [Debt; 3] = [Debt::new(), Debt::new(), Debt::new()];

    for i in 0..3 {
        get_debt(&mut zippy[i])
    }

    for i in 0..3 {
        show_debt(&zippy[i])
    }
    write!(stdout, "Total debt: ${}\n", sum_debts(&zippy, 3));
}

fn another() {
    use namesp::pers::Person;

    let mut stdout: io::Stdout = io::stdout();

    let collector: Person = Person {
        fname: "Milo".to_string(),
        lname: "Rightshift".to_string(),
    };
    pers::show_person(&collector);
    let _: usize = stdout.write(b"\n").unwrap();
}
