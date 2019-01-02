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
use logos::Logos;

/// Runs the tokenizer on an input string and collects all of the output tokens
/// into a list, continuing past lex errors.
pub fn tokenize<S: AsRef<str>>(str: S) -> Vec<Token> {
    let mut lexer = TokenType::lexer(str.as_ref());
    let mut tokens: Vec<Token> = vec![];

    loop {
        tokens.push(Token::new(lexer.token, lexer.range()));

        if lexer.token == TokenType::Eof {
            break;
        }

        lexer.advance();
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
