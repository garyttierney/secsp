//! A whitespace and comment preserving lexer designed for use in both the compiler frontend
//! and IDEs.
//!
//! The output of the tokenizer is lightweight, and only contains information about the
//! type of a [Token] and where it occurred in the source.
//!
//! ```rust,ignore
//! use crate::token::{Token, TokenType};
//! use crate::lexer;
//!
//! let tokens = lexer::tokenize("test");
//! assert_eq!(Token(TokenType::Name, 0..4), tokens[0]);
//! assert_eq!(Token(TokenType::Eof, 4..5), tokens[1]);
//! ```

use crate::token::{Token, TokenType};
use itertools::Itertools;
use logos::Lexer;
use logos::Logos;
use logos::Source;
use std::marker::PhantomData;
use std::ops::Range;

struct Tokenizer<'a> {
    lexer: Lexer<TokenType, &'a str>,
    seen_eof: bool,
}

impl<'a> Tokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Tokenizer {
            lexer: TokenType::lexer(text),
            seen_eof: false,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = (TokenType, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.seen_eof {
            return None;
        }

        let ty = self.lexer.token;
        let range = self.lexer.range();

        self.lexer.advance();

        if ty == TokenType::Eof {
            self.seen_eof = true;
        }

        Some((ty, range))
    }
}

/// Runs the tokenizer on an input string and collects all of the output tokens
/// into a list, continuing past lex errors.
pub fn tokenize<S: AsRef<str>>(str: S) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(str.as_ref());
    let mut tokens: Vec<Token> = vec![];
    let mut iter = tokenizer.into_iter().peekable();

    while let Some((token, range)) = iter.next() {
        match token {
            TokenType::Illegal => {
                let leading_range = iter
                    .peeking_take_while(|(ty, _)| *ty == TokenType::Illegal)
                    .map(|(_, range)| range)
                    .last()
                    .unwrap_or(range.clone());

                tokens.push(Token::new(TokenType::Illegal, range.start..leading_range.end))
            }
            _ => tokens.push(Token::new(token, range)),
        }
    }

    tokens
}

#[test]
fn preserves_whitespace() {
    let types: Vec<TokenType> = tokenize("test abc 123")
        .into_iter()
        .map(|t| t.into())
        .collect();

    assert_eq!(
        vec![
            TokenType::Name,
            TokenType::Whitespace,
            TokenType::Name,
            TokenType::Whitespace,
            TokenType::Integer,
            TokenType::Eof,
        ],
        types
    );
}
