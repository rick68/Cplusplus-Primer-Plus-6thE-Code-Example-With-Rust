// equal.rs -- equality vs assignment

use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let quizscores: [i32; 10] = [20, 20, 20, 20, 20, 19, 20, 18, 20, 20];

    let _: usize = stdout.write(b"Doing it right:\n")?;
    for i in 0.. {
        if !(quizscores[i] == 20) {
            break;
        }
        write!(stdout, "quiz {} is a 20\n", i);
    }

    Ok(())
}
