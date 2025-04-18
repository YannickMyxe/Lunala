use crate::parser::Parser;
use crate::tokens::TokenType;
use crate::tree::Expression::{Binary, Grouping, Unary};
use crate::tree::{Expression, Literal};
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod tokens;
mod parser;
mod errors;
mod tree;

fn main() -> Result<(), errors::LunalaErrors > {
    println!("Starting Lunala!");
    let path = Path::new("./Lunala/examples/hello.luna");
    let mut file = File::open(&path).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("couldn't read file");

    let mut parser = Parser::new(contents.as_str());
    let tokens = parser.scan_tokens()?;

    println!("\nTokens:[ ");
    for token in tokens {
        println!("  {}", token);
    }
    println!("]");

    let tree = Binary {
        operator: TokenType::Star,
        left: Box::from(Unary {
            operator: TokenType::Minus,
            expression: Box::from(Expression::Literal(Literal::Number(32)))
        }),
        right: Box::from(
            Grouping {
                expression: Box::from(Expression::Literal(Literal::Number(32))),
            }
        )
    };
    println!("\nExpression: {:#?}", tree);

    Ok(())
}
