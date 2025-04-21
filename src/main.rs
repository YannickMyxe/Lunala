use crate::scanner::Scanner;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::errors::LunalaErrors;

mod tokens;
mod scanner;
mod errors;
mod tree;
mod parser;

fn main() -> Result<(), LunalaErrors> {
    println!("Starting Lunala!");
    let mut contents = String::new();
    /*
    let path = Path::new("./Lunala/examples/hello.luna");
    let mut file = File::open(&path).expect("Couldn't open file");

    file.read_to_string(&mut contents).expect("couldn't read file");
    
     */
    contents.push_str("2 + ( 5 - 3 ) - 1");

    let mut scanner = Scanner::new(&contents);
    let mut parser = parser::Parser::new(scanner.scan_tokens()?);
    
    let expr = parser.parse()?;
    
    println!("Exp=> {}", expr);

    
    Ok(())
}
