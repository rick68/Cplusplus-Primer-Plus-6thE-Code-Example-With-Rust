// myfirst.rs -- displays a message

use std::io::{self, Write};                                  // make definitions visible

fn main() -> io::Result<()>                                  // function header
{                                                            // start of function body

    io::stdout().write(b"Come up and Rust me some time.")?;  // message
    io::stdout().write(b"\n")?;                              // start a new line
    io::stdout().write(b"You won't regret it!")?;            // more output

    Ok(())                                                   // terminate main()
}                                                            // end of function body
