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

    fn next(&mut self) {
        self.position += 1;
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
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, min_bp: u8) -> ParserResult<SyntaxTree> {
        let mut left = self.parse_literal()?;

        loop {
            let token = self.peek(0);

            match token.kind {
                TokenKind::EndOfFile => break,
                _ => (),
            }

            let op = match self.peek(0).kind.is_binary_operator() {
                Some(op) => Ok(op),
                None => Err(ParserError::UnexpectedToken),
            }?;

            let (l_bp, r_bp) = op.precedence();

            if l_bp < min_bp {
                break;
            }

            self.next();

            let right = self.parse_expression(r_bp)?;

            left = SyntaxTree::BinOp(op, Box::new(left), Box::new(right))
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
