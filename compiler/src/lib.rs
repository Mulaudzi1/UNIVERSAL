pub mod ast;
pub mod diagnostic;
pub mod interpreter;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod semantic;
pub mod token;
pub mod types;

use diagnostic::Diagnostic;
use interpreter::{Interpreter, RuntimeOutput};
use lexer::Lexer;
use parser::Parser;
use semantic::Analyzer;

pub fn check(source: &str) -> Result<ast::Program, Vec<Diagnostic>> {
    let tokens = Lexer::new(source).lex()?;
    let program = Parser::new(tokens).parse()?;
    Analyzer::new().analyze(&program)?;
    Ok(program)
}

pub fn run(source: &str) -> Result<RuntimeOutput, Vec<Diagnostic>> {
    let program = check(source)?;
    Interpreter::new().run(&program)
}
