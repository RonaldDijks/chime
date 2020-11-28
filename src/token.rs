use crate::syntax_tree::BinOpKind;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // Special tokens
    EndOfFile,
    BadToken,
    Whitespace,

    // Float literals
    FloatLiteral,

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl TokenKind {
    pub fn is_binary_operator(&self) -> Option<BinOpKind> {
        match self {
            TokenKind::Plus => Some(BinOpKind::Add),
            TokenKind::Minus => Some(BinOpKind::Sub),
            TokenKind::Asterisk => Some(BinOpKind::Mul),
            TokenKind::Slash => Some(BinOpKind::Div),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}
