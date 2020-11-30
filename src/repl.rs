use crate::{evaluator::Evaluator, parser::Parser};

pub fn repl() {
    loop {
        let mut text = String::new();
        std::io::stdin().read_line(&mut text).unwrap();
        let mut parser = Parser::new(text);
        let syntax_tree = parser.parse().unwrap();
        println!("syntax_tree: {:?}", syntax_tree);
        let evaluator = Evaluator::new();
        let result = evaluator.evaluate(&syntax_tree);
        println!("result: {:?}", result);
    }
}
