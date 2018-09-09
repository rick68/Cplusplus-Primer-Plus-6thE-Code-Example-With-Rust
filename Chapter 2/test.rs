        use
            std
    ::          io; use std::
io ::
    prelude ::
        *
;

    fn
main
() -> io::
            Result<(
        )> {
            let
        mut
    stdout:
        io::Stdout =
            io::stdout();
                stdout
                    .write(
b"Come up and C++ me some time.\n"
)?;     stdout.write(
b"\n")?; stdout.
    write(
b"You won't regret it!"
)?; Ok(()) }
