use crate::syntax_tree::BinOpKind;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // Special tokens
    EndOfFile,
    BadToken,
    Whitespace,
    LeftParenthesis,
    RightParenthesis,

    // Float literals
    FloatLiteral,

    // Keywords
    True,
    False,
    Identifier,

    // Operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    PipePipe,
    AmpersandAmpersand,
}

impl TokenKind {
    pub fn is_binary_operator(&self) -> Option<BinOpKind> {
        match self {
            TokenKind::Plus => Some(BinOpKind::Add),
            TokenKind::Minus => Some(BinOpKind::Sub),
            TokenKind::Asterisk => Some(BinOpKind::Mul),
            TokenKind::Slash => Some(BinOpKind::Div),
            TokenKind::PipePipe => Some(BinOpKind::LogicalOr),
            TokenKind::AmpersandAmpersand => Some(BinOpKind::LogicalAnd),
            _ => None,
        }
    }

    pub fn keyword(text: &str) -> TokenKind {
        match text {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Identifier,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}
