// strquote.rs -- different designs

use std::io;
use std::io::prelude::*;
use std::string::String;
use std::str;
use std::option::Option;

fn main() -> io::Result<()> {
    let mut stdout: io::Stdout = io::stdout();
    let stdin: io::Stdin = io::stdin();

    let mut input: String = String::new();
    let copy: String;
    let mut result: String;

    let _: usize = stdout.write(b"Enter a string: ")?;
    stdout.flush()?;
    let _: usize = stdin.read_line(&mut input)?;
    let _: Option<char> = input.pop(); // remove newline
    copy = input.clone();
    write!(stdout, "Your string as entered: {}\n", input)?;
    result = version1(&input, "***");
    write!(stdout, "Your string enhanced: {}\n", result)?;
    write!(stdout, "Your original string: {}\n", input)?;

    result = version2(&mut input, "###").to_string();
    write!(stdout, "Your string enhanced: {}\n", result)?;
    write!(stdout, "Your original string: {}\n", input)?;

    write!(stdout, "Resetting original string.\n");
    input = copy;
    write!(stdout, "Your original string: {}\n", input)?;

    Ok(())
}

fn version1(s1: &String, s2: &str) -> String {
    let mut temp: String = s2.to_string();

    temp.push_str(s1.as_str());
    temp.push_str(s2);
    temp
}

fn version2<'a>(s1: &'a mut String, s2: &str) -> &'a String { // has side effect
    let temp: String = s1.to_string();
    s1.clear();
    s1.push_str(s2);
    s1.push_str(&temp);
    s1.push_str(s2);
    s1
}
