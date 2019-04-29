//! A whitespace and comment preserving lexer designed for use in both the compiler frontend
//! and IDEs.
//!
//! The output of the tokenizer is lightweight, and only contains information about the
//! type of a [Token] and where it occurred in the source.

use std::ops::Range;

use itertools::Itertools;
use logos::Lexer;
use logos::Logos;

use secsp_parser::syntax::TokenKind;

use crate::token::Token;

struct Tokenizer<'a> {
    lexer: Lexer<TokenKind, &'a str>,
    seen_eof: bool,
}

impl<'a> Tokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Tokenizer {
            lexer: TokenKind::lexer(text),
            seen_eof: false,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = (TokenKind, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.seen_eof {
            return None;
        }

        let ty = self.lexer.token;
        let range = self.lexer.range();

        self.lexer.advance();

        if ty == TokenKind::Eof {
            self.seen_eof = true;
        }

        Some((ty, range))
    }
}

/// Runs the tokenizer on an input string and collects all of the output tokens
/// into a list, continuing past lex errors.
pub fn tokenize<S: AsRef<str>>(str: S) -> Vec<Token> {
    let tokenizer = Tokenizer::new(str.as_ref());
    let mut tokens: Vec<Token> = vec![];
    let mut iter = tokenizer.peekable();

    while let Some((token, range)) = iter.next() {
        match token {
            TokenKind::Illegal | TokenKind::Whitespace | TokenKind::LineComment => {
                let range = iter
                    .peeking_take_while(|(ty, _)| *ty == token)
                    .map(|(_, range)| range)
                    .last()
                    .map_or(range.clone(), |to| range.start..to.end);

                tokens.push(Token::new(token, range))
            }
            _ => tokens.push(Token::new(token, range)),
        }
    }

    tokens
}

#[test]
fn preserves_whitespace() {
    let types: Vec<TokenKind> = tokenize("test abc 123")
        .into_iter()
        .map(|t| t.into())
        .collect();

    assert_eq!(
        vec![
            TokenKind::Name,
            TokenKind::Whitespace,
            TokenKind::Name,
            TokenKind::Whitespace,
            TokenKind::Integer,
            TokenKind::Eof,
        ],
        types
    );
}
