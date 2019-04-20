use std::ops::Range;

use crate::parser::syntax::TokenKind;

/// A single unit of output by the lexer.
#[derive(Copy, Clone, Debug)]
pub struct Token(pub(crate) TokenKind, pub(crate) usize, pub(crate) usize);

impl Token {
    pub fn new(ty: TokenKind, range: Range<usize>) -> Self {
        Token(ty, range.start, range.end)
    }

    pub fn ty(&self) -> TokenKind {
        self.0
    }

    pub fn range(&self) -> Range<usize> {
        self.1..self.2
    }

    pub fn is_trivia(&self) -> bool {
        match self.0 {
            TokenKind::LineComment | TokenKind::Whitespace => true,
            _ => false,
        }
    }
}

impl Into<TokenKind> for Token {
    fn into(self) -> TokenKind {
        self.0
    }
}
