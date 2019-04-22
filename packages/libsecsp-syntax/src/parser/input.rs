use std::fmt::Debug;
use std::ops::Range;

use crate::parser::syntax::TokenKind;
use crate::token::Token;

pub trait SyntaxKindBase: Sized + PartialEq + Eq + Debug + Copy + Send + Sync {
    fn eof() -> Self;

    fn has_attached_trivia(&self) -> bool;

    fn is_whitespace(&self) -> bool;

    fn is_root(&self) -> bool;

    fn is_trivia(&self) -> bool;
}

pub trait TokenBase: Sized + Copy {
    fn kind(&self) -> rowan::SyntaxKind;

    fn is_trivia(&self) -> bool;

    fn is_whitespace(&self) -> bool;

    fn range(&self) -> Range<usize>;
}

impl TokenBase for Token {
    fn kind(&self) -> rowan::SyntaxKind {
        rowan::SyntaxKind(self.0 as u16)
    }

    fn is_trivia(&self) -> bool {
        match self.0 {
            TokenKind::LineComment | TokenKind::Whitespace => true,
            _ => false,
        }
    }

    fn is_whitespace(&self) -> bool {
        match self.0 {
            TokenKind::Whitespace => true,
            _ => false,
        }
    }

    fn range(&self) -> Range<usize> {
        self.1..self.2
    }
}

impl<'a, T: TokenBase> ParserInput<'a, T> {
    pub fn new(text: &'a str, tokens: &'a [T]) -> Self {
        let non_trivia_tokens = tokens
            .iter()
            .filter(|tok| !tok.is_trivia())
            .cloned()
            .collect();

        ParserInput {
            text,
            tokens: non_trivia_tokens,
        }
    }

    pub fn kind(&self, idx: usize) -> rowan::SyntaxKind {
        if idx >= self.tokens.len() {
            return rowan::SyntaxKind(TokenKind::Eof as u16);
        }

        self.tokens[idx].kind()
    }

    pub fn text(&self, idx: usize) -> &'a str {
        if idx >= self.tokens.len() {
            return "";
        }

        let tok = &self.tokens[idx];
        let range = tok.range();

        &self.text[range]
    }
}

pub struct ParserInput<'a, T: TokenBase> {
    text: &'a str,
    tokens: Vec<T>,
}
