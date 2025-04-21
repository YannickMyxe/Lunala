use crate::errors::LunalaErrors;
use crate::interpreter::{Interpreter, Object};
use crate::scanner::Scanner;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

    let mut contents = String::new();
    if !args.is_empty() {
        handle_file(&mut contents);
    } else {
        handle_repl(&mut contents);
    }

    let mut scanner = Scanner::new(&contents);
    let mut parser = parser::Parser::new(scanner.scan_tokens()?);
    
    let expr = parser.parse()?;
    println!("Exp=> {}", expr);
    
    let obj = Interpreter::interpret(expr)?;
    println!("[Lunala]> {}", obj);
    
    Ok(())
}

fn handle_file(buffer: &mut String) {
    let path = Path::new("./Lunala/examples/hello.luna");
    let mut file = File::open(path).expect("Couldn't open file");

    file.read_to_string(buffer).expect("couldn't read file");
}

fn handle_repl(buffer: &mut String) {
    buffer.push_str("-2 + ( 8.5 - 3.5 ) - 4");
    //buffer.push_str("-5 + 15");
    //buffer.push_str(" 5 <= 8 ");
    //buffer.push_str("((((((((((3 + 5) + 8) + 14) + 8) * 8) / 3) + 9) - 7) + 5) - 9)");
    println!("Lunala> {}", buffer.clone());
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
    assert_eq!(obj, Object::Number(7f32), "Interpret failed: {:?}", obj);
    
    println!("[Test success] {:?}", obj);
}