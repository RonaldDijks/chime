use crate::token::{Token, TokenKind};

pub struct Lexer {
    position: usize,
    text: String,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self { position: 0, text }
    }

    fn peek(&self, offset: usize) -> char {
        match self.text.chars().nth(self.position + offset) {
            Some(character) => character,
            None => '\0',
        }
    }

    fn current(&self) -> char {
        self.peek(0)
    }

    pub fn next(&mut self) -> Token {
        let start = self.position;

        let kind = if self.current().is_whitespace() {
            while self.current().is_whitespace() {
                self.position += 1;
            }
            TokenKind::Whitespace
        } else if self.current().is_numeric() {
            while self.current().is_numeric() {
                self.position += 1;
            }
            TokenKind::FloatLiteral
        } else if self.current() == '\0' {
            TokenKind::EndOfFile
        } else if self.current() == '+' {
            self.position += 1;
            TokenKind::Plus
        } else if self.current() == '-' {
            self.position += 1;
            TokenKind::Minus
        } else if self.current() == '*' {
            self.position += 1;
            TokenKind::Asterisk
        } else if self.current() == '/' {
            self.position += 1;
            TokenKind::Slash
        } else if self.current() == '(' {
            self.position += 1;
            TokenKind::LeftParenthesis
        } else if self.current() == ')' {
            self.position += 1;
            TokenKind::RightParenthesis
        } else if self.current() == '|' && self.peek(1) == '|' {
            self.position += 2;
            TokenKind::PipePipe
        } else if self.current() == '&' && self.peek(1) == '&' {
            self.position += 2;
            TokenKind::AmpersandAmpersand
        } else if self.current().is_alphabetic() {
            while self.current().is_alphanumeric() {
                self.position += 1;
            }
            let end = self.position;
            let text = &self.text[start..end];
            TokenKind::keyword(text)
        } else {
            TokenKind::BadToken
        };
        let end = self.position;
        let text = self.text[start..end].into();
        Token { kind, text }
    }
}
