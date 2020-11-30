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
                tokens.push(token);
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

    fn next(&mut self) -> Token {
        let token = self.current();
        self.position += 1;
        token
    }

    fn expect(&mut self, expected: TokenKind) -> ParserResult<Token> {
        let actual = self.current();

        if actual.kind == expected {
            self.position += 1;
            Ok(actual)
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    pub fn parse(&mut self) -> ParserResult<SyntaxTree> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> ParserResult<SyntaxTree> {
        if self.peek(0).kind == TokenKind::Identifier && self.peek(1).kind == TokenKind::Equals {
            let identifier = self.next();
            let _operator = self.next();
            let right = self.parse_assignment_expression()?;
            return Ok(SyntaxTree::Assignment(identifier.text, Box::new(right)));
        }
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, min_bp: u8) -> ParserResult<SyntaxTree> {
        let mut left = self.parse_primary_statement()?;

        loop {
            let token = self.current();

            let op = match token.kind {
                TokenKind::EndOfFile => break,
                TokenKind::RightParenthesis => break,
                _ => match token.kind.is_binary_operator() {
                    Some(op) => Ok(op),
                    None => Err(ParserError::UnexpectedToken),
                },
            }?;

            let (l_bp, r_bp) = op.precedence();

            if l_bp < min_bp {
                break;
            }

            self.next();

            let right = self.parse_binary_expression(r_bp)?;

            left = SyntaxTree::BinOp(op, Box::new(left), Box::new(right))
        }

        Ok(left)
    }

    fn parse_primary_statement(&mut self) -> ParserResult<SyntaxTree> {
        let value = self.current();
        match value.kind {
            TokenKind::LeftParenthesis => self.parse_parenthesised_expression(),
            TokenKind::FloatLiteral => self.parse_float_literal(),
            TokenKind::True => self.parse_boolean_literal(),
            TokenKind::False => self.parse_boolean_literal(),
            TokenKind::Identifier => self.parse_identifier(),
            _ => Err(ParserError::UnexpectedToken),
        }
    }

    fn parse_parenthesised_expression(&mut self) -> ParserResult<SyntaxTree> {
        self.expect(TokenKind::LeftParenthesis)?;
        let expr = self.parse_binary_expression(0)?;
        self.expect(TokenKind::RightParenthesis)?;
        Ok(expr)
    }

    fn parse_float_literal(&mut self) -> ParserResult<SyntaxTree> {
        let value = self
            .expect(TokenKind::FloatLiteral)?
            .text
            .parse()
            .map_err(|_| ParserError::FloatLiteralParse)?;
        Ok(SyntaxTree::F64(value))
    }

    fn parse_boolean_literal(&mut self) -> ParserResult<SyntaxTree> {
        let token = self.current();

        let syntax_tree = match token.kind {
            TokenKind::True => Ok(SyntaxTree::Bool(true)),
            TokenKind::False => Ok(SyntaxTree::Bool(false)),
            _ => Err(ParserError::UnexpectedToken),
        };

        if syntax_tree.is_ok() {
            self.next();
        }

        syntax_tree
    }

    fn parse_identifier(&mut self) -> ParserResult<SyntaxTree> {
        let token = self.expect(TokenKind::Identifier)?;
        let syntax_tree = SyntaxTree::Identifier(token.text);
        Ok(syntax_tree)
    }
}
