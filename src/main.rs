use crate::errors::LunalaErrors;
use crate::interpreter::{Interpreter};
use crate::scanner::Scanner;
use std::fs::File;
use std::io;
use std::io::{stdout, Read, Write};
use std::path::Path;

mod tokens;
mod scanner;
mod errors;
mod expressions;
mod parser;
mod interpreter;
mod statement;

fn main() -> Result<(), LunalaErrors> {
    println!("[Lunala]");
    
    // Collect Command-Line arguments and skip the first one (programPath)
    let args: Vec<String> = std::env::args().skip(1).collect();
    
    if !args.is_empty() {
        handle_file()?;
    } else {
        handle_repl()?;
    }
    
    Ok(())
}

fn interpret(buffer: &str) -> Result<(), LunalaErrors> {
    let mut scanner = Scanner::new(buffer);
    let mut parser = parser::Parser::new(scanner.scan_tokens()?);

    let expressions = parser.parse()?;
    Interpreter::interpret(expressions)?;

    Ok(())
}

fn handle_file() -> Result<(), LunalaErrors> {
    let buffer = &mut String::new();
    let path = Path::new("./Lunala/examples/hello.luna");
    let mut file = File::open(path).expect("Couldn't open file");

    file.read_to_string(buffer).expect("couldn't read file");
    interpret(buffer)?;
    Ok(())
}

fn handle_repl() -> Result<(), LunalaErrors> {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        print!("Lunala REPL> ");
        let _ = stdout().flush();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {},
            Err(err) => {
                eprintln!("{}", err);
            }
        }
        match buffer.as_str().trim() {
            "QUIT" => { return Ok(()) }
            _ => {
                interpret(&buffer)?;
            }
        }
        
    }
}