pub mod ast;
pub mod lexer;
pub mod parser;
pub mod table;
pub mod token;

use std::{env, fs};

use ast::AST;
use lexer::Lexer;
use parser::Parser;
use token::Token;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];
    let source_code = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Unable to read file: {}", e);
            return;
        }
    };

    if source_code.len() == 0 {
        println!("Empty source file - no parsing needed");
        return;
    }
    // println!("{}", source_code);
    let lexer = Lexer::new(&source_code);
    let tokens = lexer.collect::<Vec<Token>>();

    let mut parser = Parser::new(tokens);
    let tree = parser.parse();

    // dbg!(tree.clone());
    let mut ast = AST::new(tree);
    ast.prune();
    ast.to_xml();
}
