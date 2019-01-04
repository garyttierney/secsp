use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;

use crate::ast::SyntaxKind;
use crate::token::Token;
use crate::token::TokenType;

pub trait SyntaxKindBase: Sized + PartialEq + Eq + Debug + Copy + Send + Sync {
    fn eof() -> Self;

    fn has_attached_trivia(&self) -> bool;

    fn is_whitespace(&self) -> bool;

    fn is_root(&self) -> bool;

    fn is_trivia(&self) -> bool;
}

impl SyntaxKindBase for SyntaxKind {
    fn eof() -> Self {
        SyntaxKind::Token(TokenType::Eof)
    }

    fn has_attached_trivia(&self) -> bool {
        match self {
            SyntaxKind::Container | SyntaxKind::MacroDef | SyntaxKind::Variable => true,
            _ => false,
        }
    }

    fn is_whitespace(&self) -> bool {
        *self == SyntaxKind::Token(TokenType::Whitespace)
    }

    fn is_root(&self) -> bool {
        *self == SyntaxKind::SourceFile
    }

    fn is_trivia(&self) -> bool {
        match self {
            SyntaxKind::Token(TokenType::LineComment)
            | SyntaxKind::Token(TokenType::Whitespace) => true,
            _ => false,
        }
    }
}

pub trait TokenBase<S: SyntaxKindBase>: Sized + Copy {
    fn kind(&self) -> S;

    fn is_trivia(&self) -> bool;

    fn range(&self) -> Range<usize>;
}

impl TokenBase<SyntaxKind> for Token {
    fn kind(&self) -> SyntaxKind {
        SyntaxKind::Token(self.0)
    }

    fn is_trivia(&self) -> bool {
        match self.0 {
            TokenType::LineComment | TokenType::Whitespace => true,
            _ => false,
        }
    }

    fn range(&self) -> Range<usize> {
        self.1..self.2
    }
}

pub struct ParserInput<'a, K: SyntaxKindBase, T: TokenBase<K>> {
    text: &'a str,
    tokens: Vec<T>,
    pos: usize,
    _phantom: PhantomData<K>,
}

impl<'a, K: SyntaxKindBase, T: TokenBase<K>> ParserInput<'a, K, T> {
    pub fn new(text: &'a str, tokens: &'a [T]) -> Self {
        let non_trivia_tokens = tokens
            .iter()
            .filter(|tok| !tok.is_trivia())
            .map(|tok| *tok)
            .collect();

        ParserInput {
            text,
            tokens: non_trivia_tokens,
            pos: 0,
            _phantom: PhantomData,
        }
    }

    pub fn kind(&self, idx: usize) -> K {
        if idx >= self.tokens.len() {
            return K::eof();
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
