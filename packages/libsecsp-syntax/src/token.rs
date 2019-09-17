use std::ops::Range;

use rowan::TextUnit;

use secsp_parser::syntax::SyntaxKind;
use secsp_parser::syntax::TokenKind;

/// A single unit of output by the lexer.
#[derive(Copy, Clone, Debug)]
pub struct Token(pub(crate) TokenKind, pub(crate) usize, pub(crate) usize);

impl Token {
    pub fn new(ty: TokenKind, range: Range<usize>) -> Self {
        Token(ty, range.start, range.end)
    }

    pub fn kind(&self) -> SyntaxKind {
        self.0.syntax_kind()
    }

    pub fn range(&self) -> Range<usize> {
        self.1..self.2
    }

    pub fn len(&self) -> TextUnit {
        TextUnit::from_usize(self.2 - self.1)
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
