use crate::syntax_tree::BinOpKind;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // Special tokens
    EndOfFile,
    BadToken,
    Whitespace,

    // Float literals
    FloatLiteral,

    // Boolean Literals
    True,
    False,

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

    pub fn is_keyword(text: &str) -> Option<TokenKind> {
        match text {
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}
