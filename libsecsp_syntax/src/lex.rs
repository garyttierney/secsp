use super::codemap::Span;

use std::str;
use std::iter;

#[derive(Clone, Debug, PartialEq)]
/// A structural code delimiter that can occur in source code.
pub enum DelimiterType {
    /// A curly brace delimiter.  i.e., '{' or '}'.
    Brace,

    /// A parenthesis delimiter.  i.e., '(' or ')'.
    Parenthesis,
}

/// A single lexical unit in the CSP grammar.
#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    /// A name identifier token, containing a reference to the original source data.
    Name(&'a str),

    /// A string literal token, containing a reference to the original source data.
    Literal(&'a str),

    /// An opening delimiter token of the given `DelimiterType`.
    OpenDelimiter(DelimiterType),

    /// A closing delimiter token of the given `DelimiterType`.
    CloseDelimiter(DelimiterType),

    /// An unrecognized character that is illegal in the grammar.
    Illegal(char),

    /// The semicolon token. Used to terminate statements.
    Semicolon,

    /// The period token.  Used to separate fully qualified names.
    Dot,

    /// The bitwise binary AND operator: `&`.
    BitwiseAnd,

    /// The bitwise binary OR operator: `|`.
    BitwiseOr,

    /// The bitwise binary XOR operator: `^`.
    BitwiseXor,

    /// The bitwise unary NOT operator: `~`.
    BitwiseNot,

    /// The logical AND operator: `&&`.
    LogicalAnd,

    /// The logical OR operator: `||`.
    LogicalOr,

    /// The logical unary NOT operator: `!`.
    LogicalNot,

    /// The pipe-equals operator, used for flipping on bits in bitsets.
    SetModifier,
}

/// A representation of a `Token` and it's locational `Span` information.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenAndSpan<'a> {
    pub span: Span,
    pub token: Token<'a>,
}

impl<'a> TokenAndSpan<'a> {
    pub fn new(token: Token<'a>, span: Span) -> Self {
        TokenAndSpan { token, span }
    }
}

/// A lexer that takes a reference to a `str` slice as input
/// and implements an `Iterator` over lexed `Token`s with their respective
/// `Span` information.
///
/// Example:
///
/// ```rust
/// use secsp_syntax::Span;
/// use secsp_syntax::lex::{Token, TokenAndSpan, Tokenizer};
///
/// let tokenizer = Tokenizer::new("a ; ab");
/// let tokens : Vec<TokenAndSpan> = tokenizer.collect();
///
/// assert_eq!(
///     vec![
///         TokenAndSpan::new(Token::Name("a"), Span::at(0)),
///         TokenAndSpan::new(Token::Semicolon, Span::at(2)),
///         TokenAndSpan::new(Token::Name("ab"), Span::from(4, 5))
///     ],
///     tokens
/// );
/// ```
pub struct Tokenizer<'a> {
    /// A reference to the source data.
    input: &'a str,

    /// A peekable iterator over characters in the source data.
    iter: iter::Peekable<str::Chars<'a>>,

    /// The position of the current character in the input iterator.
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    /// Create a new `Tokenizer` over the given `input` string.
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input,
            iter: input.chars().peekable(),
            pos: 0,
        }
    }

    /// Peek the next character from the input iterator and dereference it from a `&char` to `char`.
    fn peek(&mut self) -> Option<char> {
        self.iter.peek().cloned()
    }

    /// Consume a character from the input iterator and advance the `Tokenizer`s position
    /// if available.
    fn take(&mut self) -> Option<char> {
        let current = match self.iter.next() {
            Some(ch) => ch,
            None => return None,
        };

        self.pos += 1;

        Some(current)
    }

    /// Start from the last `char` lexed and tokenize all valid name characters,
    /// returning a `Token::Name` with a reference to the source data.
    fn name(&mut self) -> TokenAndSpan<'a> {
        let start_pos = self.pos - 1;

        loop {
            let next = self.iter.peek().cloned();

            match next {
                Some(ch) if ch.is_alphanumeric() || ch == '_' => self.take(),
                Some(_) | None => break,
            };
        }

        let end_pos = self.pos;

        TokenAndSpan::new(
            Token::Name(&self.input[start_pos..end_pos]),
            Span::from(start_pos, end_pos - 1),
        )
    }

    /// Create and return a new `TokenAndSpan` occupying a single character of the given `Token`.
    fn term(&self, token: Token<'a>) -> TokenAndSpan<'a> {
        self.term_from(self.pos, token)
    }

    /// Create and return a new `TokenAndSpan` occupying a range of characters from `start`
    /// to the `Tokenizer`s current position.
    fn term_from(&self, start: usize, token: Token<'a>) -> TokenAndSpan<'a> {
        TokenAndSpan {
            token,
            span: Span {
                start: start - 1,
                end: self.pos - 1,
            },
        }
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenAndSpan<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = match self.take() {
                Some(ch) => ch,
                None => return None,
            };
            let pos = self.pos;

            match current {
                ';' => return Some(self.term(Token::Semicolon)),
                '.' => return Some(self.term(Token::Dot)),
                '(' => return Some(self.term(Token::OpenDelimiter(DelimiterType::Parenthesis))),
                ')' => return Some(self.term(Token::CloseDelimiter(DelimiterType::Parenthesis))),
                '{' => return Some(self.term(Token::OpenDelimiter(DelimiterType::Brace))),
                '}' => return Some(self.term(Token::CloseDelimiter(DelimiterType::Brace))),
                '^' => return Some(self.term(Token::BitwiseXor)),
                '~' => return Some(self.term(Token::BitwiseNot)),
                '!' => return Some(self.term(Token::LogicalNot)),
                '&' => match self.peek() {
                    Some('&') => {
                        self.take();
                        return Some(self.term_from(pos, Token::LogicalAnd));
                    }
                    _ => return Some(self.term(Token::BitwiseAnd)),
                },
                '|' => match self.peek() {
                    Some('|') => {
                        self.take();
                        return Some(self.term_from(pos, Token::LogicalOr));
                    }
                    Some('=') => {
                        self.take();
                        return Some(self.term_from(pos, Token::SetModifier));
                    }
                    _ => return Some(self.term(Token::BitwiseOr)),
                },
                'a'...'z' | 'A'...'Z' => return Some(self.name()),
                c if c.is_whitespace() => continue,
                c => return Some(self.term(Token::Illegal(c))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn lex_braces() {
        let tokenizer = Tokenizer::new("{ }");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![
                TokenAndSpan::new(Token::OpenDelimiter(DelimiterType::Brace), Span::at(0)),
                TokenAndSpan::new(Token::CloseDelimiter(DelimiterType::Brace), Span::at(2)),
            ],
            tokens
        );
    }

    #[test]
    pub fn lex_parens() {
        let tokenizer = Tokenizer::new("( )");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![
                TokenAndSpan::new(
                    Token::OpenDelimiter(DelimiterType::Parenthesis),
                    Span::at(0),
                ),
                TokenAndSpan::new(
                    Token::CloseDelimiter(DelimiterType::Parenthesis),
                    Span::at(2),
                ),
            ],
            tokens
        );
    }

    #[test]
    pub fn lex_name() {
        let tokenizer = Tokenizer::new("my_ident");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![TokenAndSpan::new(Token::Name("my_ident"), Span::from(0, 7))],
            tokens
        );
    }

    #[test]
    pub fn lex_logical_ops() {
        let tokenizer = Tokenizer::new("&& || !");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![
                TokenAndSpan::new(Token::LogicalAnd, Span::from(0, 1)),
                TokenAndSpan::new(Token::LogicalOr, Span::from(3, 4)),
                TokenAndSpan::new(Token::LogicalNot, Span::at(6)),
            ],
            tokens
        );
    }

    #[test]
    pub fn lex_bitwise_ops() {
        let tokenizer = Tokenizer::new("& | ~");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![
                TokenAndSpan::new(Token::BitwiseAnd, Span::at(0)),
                TokenAndSpan::new(Token::BitwiseOr, Span::at(2)),
                TokenAndSpan::new(Token::BitwiseNot, Span::at(4)),
            ],
            tokens
        );
    }

    #[test]
    pub fn lex_set_modifier() {
        let tokenizer = Tokenizer::new("|=");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![TokenAndSpan::new(Token::SetModifier, Span::from(0, 1))],
            tokens
        );
    }

    #[test]
    pub fn lex_dot() {
        let tokenizer = Tokenizer::new(".");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(vec![TokenAndSpan::new(Token::Dot, Span::at(0))], tokens);
    }

    #[test]
    pub fn lex_illegal() {
        let tokenizer = Tokenizer::new("$");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![TokenAndSpan::new(Token::Illegal('$'), Span::at(0))],
            tokens
        );
    }

    #[test]
    pub fn lex_semicolon() {
        let tokenizer = Tokenizer::new(";");
        let tokens: Vec<TokenAndSpan> = tokenizer.collect();

        assert_eq!(
            vec![TokenAndSpan::new(Token::Semicolon, Span::at(0))],
            tokens
        );
    }
}
