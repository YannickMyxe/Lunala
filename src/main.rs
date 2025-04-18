use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::parser::Parser;

mod tokens;
mod parser;
mod errors;

fn main() -> Result<(), errors::LunalaErrors > {
    println!("Starting Lunala!");
    let path = Path::new("./Lunala/examples/hello.luna");
    let mut file = File::open(&path).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("couldn't read file");

    for (line_number, line) in contents.lines().enumerate() {
        println!("[{}] => `{}`", line_number+1, line);
    }

    let mut parser = Parser::new(contents.as_str());
    let tokens = parser.scan_tokens()?;

    println!("\n\n\nTokens:[ ");
    for token in tokens {
        println!("  {}", token);
    }
    println!("]");

    Ok(())
}
