mod evaluator;
mod lexer;
mod parser;
mod syntax_tree;
mod token;

use evaluator::Evaluator;
use parser::Parser;

fn main() {
    let text = "123 + 123".into();
    let mut parser = Parser::new(text);
    let syntax_tree = parser.parse().unwrap();
    println!("syntax_tree: {:?}", syntax_tree);
    let evaluator = Evaluator::new();
    let result = evaluator.evaluate(&syntax_tree);
    println!("result: {:?}", result);
}
