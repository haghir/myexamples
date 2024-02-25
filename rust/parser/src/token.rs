mod span;

use span::Span;

pub enum TokenKind {
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Number(i32),
    Ident(usize),
}

pub struct Token {
    tokenKind: TokenKind,
    span: Span,
}
