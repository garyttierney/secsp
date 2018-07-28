pub mod text_reader;
pub mod token;
pub mod token_cursor;
pub mod token_tree;

pub use codespan::ByteIndex;
pub use codespan::ByteSpan;
pub use codespan::Span;

pub use self::text_reader::*;
pub use self::token::*;
pub use self::token_tree::*;

use codespan::FileMap;
use codespan_reporting::Severity;
use std::borrow::Borrow;
use std::sync::Arc;

use crate::parse::ParseResult;
use crate::session::ParseSession;

impl<'input> TextReader<'input> {
    pub fn read_next_token(&mut self) -> (Token, ByteSpan) {
        let next = match self.next() {
            Some(ch) => ch,
            None => {
                return (Token::Eof, ByteSpan::default());
            }
        };

        let position = next.position();
        let char = next.char();

        match char {
            ';' => (Token::Semicolon, position),
            '.' => (Token::Dot, position),
            '(' => (Token::OpenDelimiter(DelimiterType::Parenthesis), position),
            ')' => (Token::CloseDelimiter(DelimiterType::Parenthesis), position),
            '{' => (Token::OpenDelimiter(DelimiterType::Brace), position),
            '}' => (Token::CloseDelimiter(DelimiterType::Brace), position),
            '^' => (Token::BitwiseXor, position),
            '~' => (Token::BitwiseNot, position),
            '!' => (Token::LogicalNot, position),
            ',' => (Token::Comma, position),
            '&' => match self.peek() {
                Some(IndexedChar(next_pos, '&')) => {
                    self.next();
                    (Token::LogicalAnd, position.with_end(next_pos.end()))
                }
                _ => (Token::BitwiseAnd, position),
            },
            '|' => match self.peek() {
                Some(IndexedChar(next_pos, '|')) => {
                    self.next();
                    (Token::LogicalOr, position.with_end(next_pos.end()))
                }
                Some(IndexedChar(next_pos, '=')) => {
                    self.next();
                    (Token::SetModifier, position.with_end(next_pos.end()))
                }
                _ => (Token::BitwiseOr, position),
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut last_pos = position;

                loop {
                    let next = self.peek();

                    match next {
                        Some(IndexedChar(pos, ch)) if ch.is_alphanumeric() || ch == '_' => {
                            self.next();
                            last_pos = pos;
                        }
                        Some(_) | None => break,
                    };
                }

                let range = position.to(last_pos);
                let value = self.range(range);

                (Token::Name(value.to_owned()), range)
            }
            c if c.is_whitespace() => self.read_next_token(),
            c => (Token::Illegal(c), position),
        }
    }
}

/// A lexer that takes a reference to a `str` slice as input
/// and implements an `Iterator` over lexed `Token`s with their respective
/// `Span` information.
pub struct Tokenizer<'a, 'input> {
    reader: TextReader<'input>,
    parse_session: &'a ParseSession,
    token: Token,
    span: ByteSpan,
    open_braces: Vec<(ByteSpan, Token)>,
}

impl<'a, 'input> Tokenizer<'a, 'input> {
    /// Create a new `Tokenizer` over the given `input` string.
    pub fn new(parse_session: &'a ParseSession, file_map: &'input FileMap) -> Self {
        let mut reader = TextReader::new(file_map);
        let (token, span) = reader.read_next_token();

        Tokenizer {
            reader,
            parse_session,
            token,
            span,
            open_braces: vec![],
        }
    }

    pub fn advance(&mut self) {
        let (token, span) = self.reader.read_next_token();

        self.token = token;
        self.span = span;
    }

    pub fn tokenize(&mut self) -> ParseResult<Vec<TokenTree>> {
        let mut tts: Vec<TokenTree> = vec![];

        while self.token != Token::Eof {
            tts.push(self.tokenize_tree()?);
        }

        Ok(tts)
    }

    pub fn tokenize_tree_until_delimiter(&mut self) -> Vec<TokenTree> {
        let mut tts: Vec<TokenTree> = vec![];

        loop {
            if let Token::CloseDelimiter(_) = self.token {
                break;
            }

            match self.tokenize_tree() {
                Ok(tt) => tts.push(tt),
                Err(mut e) => {
                    e.emit();
                    break;
                }
            }
        }

        tts
    }

    pub fn tokenize_tree(&mut self) -> ParseResult<TokenTree> {
        match self.token {
            Token::Eof => {
                let msg = "this file contains an un-closed delimiter";
                let mut err = self.parse_session.diagnostic(Severity::Error, msg);

                for (span, _) in &self.open_braces {
                    err = err.span_err(*span, "did you mean to close this delimiter?");
                }

                Err(err)
            }
            Token::OpenDelimiter(delimiter) => {
                // Found the start of a delimited token tree.
                let start_sp = self.span;

                self.open_braces.push((self.span, self.token.clone()));
                self.advance();

                let tts = self.tokenize_tree_until_delimiter();
                let end_sp = self.span;

                match self.token {
                    Token::CloseDelimiter(d) if d == delimiter => {
                        // Found the matching closing delimiter.
                        self.open_braces.pop().unwrap();
                        self.advance();
                    }
                    Token::CloseDelimiter(other) => {
                        let mut err = self
                            .parse_session
                            .diagnostic(Severity::Error, "incorrect close delimiter");

                        if let Some(&(span, _)) = self.open_braces.last() {
                            err = err.span_err(span, "unclosed delimiter found here");
                        };

                        err.emit();
                        self.open_braces.pop().unwrap();

                        // If the incorrect delimiter matches an earlier opening
                        // delimiter, then don't consume it (it can be used to
                        // close the earlier one). Otherwise, consume it and
                        // report the error on the next call.
                        if !self
                            .open_braces
                            .iter()
                            .any(|(_, tok)| *tok == Token::OpenDelimiter(other))
                        {
                            self.advance();
                        }
                    }
                    _ => {}
                };

                Ok(TokenTree::Delimited(start_sp.to(end_sp), delimiter, tts))
            }
            Token::CloseDelimiter(_) => {
                let diagnostic = self
                    .parse_session
                    .diagnostic(Severity::Error, "unexpected_close_delimiter")
                    .span(self.span);

                Err(diagnostic)
            }
            _ => {
                let tt = TokenTree::Token(self.span, self.token.clone());
                self.advance();

                Ok(tt)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::lex::token_cursor::TokenCursor;
    use crate::parse::*;

    use codespan::RawIndex;
    use pretty_assertions::assert_eq;

    macro_rules! tokenizer_test {
        ($src:expr, $($span:expr => $token:expr,)*) => {{
            let sess = ParseSession::default();
            let file_map = source_to_file_map(&sess, $src);
            let token_trees = file_map_to_stream(&sess, file_map).expect("unable to tokenize");
            let cursor = TokenCursor::new(token_trees);
            let lexed_tokens: Vec<_> = cursor
                .map(|tok| (tok.span().start(), tok.token(), tok.span().end()))
                .collect();

            let expected_tokens = vec![$({
                let start = ByteIndex($span.find("~").unwrap() as RawIndex + 1);
                let end = ByteIndex($span.rfind("~").unwrap() as RawIndex + 2);
                (start, $token, end)
            }),*];

            assert_eq!(expected_tokens, lexed_tokens);
        }};
    }

    #[test]
    fn nested_delimited_tree() {
        tokenizer_test! {
            "  { abc efg {} }  ",
            "  ~               " => Token::OpenDelimiter(DelimiterType::Brace),
            "    ~~~           " => Token::Name("abc".to_owned()),
            "        ~~~       " => Token::Name("efg".to_owned()),
            "            ~     " => Token::OpenDelimiter(DelimiterType::Brace),
            "             ~    " => Token::CloseDelimiter(DelimiterType::Brace),
            "               ~  " => Token::CloseDelimiter(DelimiterType::Brace),
        };
    }

    #[test]
    fn logical_operators() {
        tokenizer_test! {
            "  ||  &&  !  ",
            "  ~~         " => Token::LogicalOr,
            "      ~~     " => Token::LogicalAnd,
            "          ~  " => Token::LogicalNot,
        }
    }

    #[test]
    fn bitwise_operators() {
        tokenizer_test! {
            "  |  &  ^  ~  ",
            "  ~           " => Token::BitwiseOr,
            "     ~        " => Token::BitwiseAnd,
            "        ~     " => Token::BitwiseXor,
            "           ~  " => Token::BitwiseNot,
        }
    }

    #[test]
    fn punctuation() {
        tokenizer_test! {
            "  ;  .  ,  ",
            "  ~        " => Token::Semicolon,
            "     ~     " => Token::Dot,
            "        ~  " => Token::Comma,
        }
    }
}
