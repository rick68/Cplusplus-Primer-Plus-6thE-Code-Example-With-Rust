// usetime2.rs -- using the third draft of the Time struct
// compile usetime2.rs and mytime2.rs together

#[macro_use]
mod mytime3;
use mytime3::*;

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();

    let aida: Time = Time_new!(3, 35);
    let tosca: Time = Time_new!(2, 48);
    let mut temp: Time;

    let _: usize = stdout.write(b"Aida and Tosca:\n")?;
    write!(stdout, "{}; {}\n", aida, tosca);
    temp = aida + tosca; // std::ops::Add::add()
    write!(stdout, "Aida + Tosca: {}\n", temp);
    temp = aida * 1.17; // std::ops::Mul::<f64>::mul()
    write!(stdout, "Aida * 1.17: {}\n", temp);
    write!(stdout, "10.0 * Tosca: {}\n", 10.0 * tosca);

    Ok(())
}
