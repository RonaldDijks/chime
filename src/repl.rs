use crate::{evaluator::Evaluator, parser::Parser};
use std::io::{self, Write};

pub fn repl() {
    let mut print_tree = false;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();

        if text == "#printTree\n" {
            print_tree = !print_tree;
        }

        let mut parser = Parser::new(text);
        let syntax_tree = parser.parse().unwrap();

        if print_tree {
            println!("syntax_tree: {:?}", syntax_tree);
        }

        let evaluator = Evaluator::new();
        let result = evaluator.evaluate(&syntax_tree);

        match result {
            Ok(value) => println!("{}", value.to_string()),
            Err(error) => println!("Error: {:?}", error),
        }
    }
}
