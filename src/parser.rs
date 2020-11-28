use crate::{
    lexer::Lexer,
    syntax_tree::SyntaxTree,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub enum ParserError {
    FloatLiteralParse,
    UnexpectedToken,
}

pub type ParserResult<A> = Result<A, ParserError>;

pub struct Parser {
    position: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(text: String) -> Self {
        let mut tokens = Vec::new();
        let mut lexer = Lexer::new(text);

        loop {
            let token = lexer.next();
            if token.kind == TokenKind::Whitespace {
                continue;
            }
            if token.kind == TokenKind::EndOfFile {
                break;
            }
            tokens.push(token);
        }

        Self {
            position: 0,
            tokens,
        }
    }

    fn peek(&self, offset: usize) -> Token {
        self.tokens
            .get(self.position + offset)
            .or(self.tokens.last())
            .unwrap()
            .clone()
    }

    fn current(&self) -> Token {
        self.peek(0)
    }

    fn expect(&mut self, expected: TokenKind) -> ParserResult<Token> {
        let actual = self.peek(0);
        if actual.kind == expected {
            self.position += 1;
            Ok(actual)
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    pub fn parse(&mut self) -> ParserResult<SyntaxTree> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParserResult<SyntaxTree> {
        let mut left = self.parse_literal()?;
        while let Some(kind) = self.current().kind.is_binary_operator() {
            self.position += 1;
            let right = self.parse_literal()?;
            left = SyntaxTree::BinOp(kind, Box::new(left), Box::new(right))
        }

        Ok(left)
    }

    fn parse_literal(&mut self) -> ParserResult<SyntaxTree> {
        let value = self
            .expect(TokenKind::FloatLiteral)?
            .text
            .parse()
            .map_err(|_| ParserError::FloatLiteralParse)?;
        Ok(SyntaxTree::F64(value))
    }
}
