mod evaluator;
mod lexer;
mod parser;
mod repl;
mod syntax_tree;
mod token;

use repl::repl;

fn main() {
    repl();
}
