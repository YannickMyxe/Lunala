use crate::errors::LunalaErrors;
use crate::interpreter::{Interpreter, Object};
use crate::scanner::Scanner;
use std::fs::File;
use std::io;
use std::io::{stdout, Read, Write};
use std::path::Path;
use crate::tree::Precision;

mod tokens;
mod scanner;
mod errors;
mod tree;
mod parser;
mod interpreter;

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

fn interpret(buffer: &String) -> Result<(), LunalaErrors> {
    let mut scanner = Scanner::new(&buffer);
    let mut parser = parser::Parser::new(scanner.scan_tokens()?);

    let expr = parser.parse()?;
    println!("Exp=> {}", expr);

    let obj = Interpreter::interpret(expr)?;
    println!("[Lunala]> {}", obj);

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

#[test]
fn test_expression_parsing() {
    // Mock tokens for the input: -2 + (8.5 - 3.5) - 4
    let contents = String::from("-2 + (8.5 - 3.5) + 4");
    let mut scanner = Scanner::new(&contents);
    let tk_opt = scanner.scan_tokens();
    assert!(tk_opt.is_ok(), "Parser failed: {:?}", tk_opt);
    let tokens = tk_opt.unwrap();
    let mut parser = parser::Parser::new(tokens);

    // Parse the expression
    let result = parser.parse();

    // Assert that the result is correct
    assert!(result.is_ok(), "Parser failed: {:?}", result);
    let expression = result.unwrap();

    let obj = Interpreter::interpret(expression).unwrap();
    let number: Precision = 7 as Precision;
    assert_eq!(obj, Object::Number(number), "Interpret failed: {:?}", obj);

    println!("[Test success] {:?}", obj);
}