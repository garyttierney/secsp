use codespan::ByteOffset;
use crate::lex::token::DelimiterType;
use crate::lex::token::Token;
use crate::lex::token_tree::TokenTree;
use crate::lex::ByteSpan;
use std::mem;

pub struct TokenAndSpan(Token, ByteSpan);

impl TokenAndSpan {
    pub fn token(&self) -> Token {
        self.0.clone()
    }

    pub fn span(&self) -> ByteSpan {
        self.1
    }

    pub fn consume(self) -> (Token, ByteSpan) {
        (self.0, self.1)
    }
}

struct TokenCursorFrame {
    open_delim: FrameDelimiter,
    close_delim: FrameDelimiter,
    tts: Vec<TokenTree>,
}

pub struct TokenCursor {
    frame: TokenCursorFrame,
    stack: Vec<TokenCursorFrame>,
}

enum FrameDelimiter {
    Token(Token, ByteSpan, bool),
    None,
}

impl FrameDelimiter {
    fn consume(&mut self) -> Option<TokenAndSpan> {
        match self {
            FrameDelimiter::Token(tok, sp, ref mut consumed @ false) => {
                *consumed = true;
                Some(TokenAndSpan(tok.clone(), *sp))
            }
            _ => None,
        }
    }

    fn from_open_delimiter(ty: DelimiterType, range: ByteSpan) -> Self {
        FrameDelimiter::Token(
            Token::OpenDelimiter(ty),
            range.with_end(range.start() + ByteOffset(1)),
            false,
        )
    }

    fn from_close_delimiter(ty: DelimiterType, range: ByteSpan) -> Self {
        FrameDelimiter::Token(
            Token::CloseDelimiter(ty),
            range.with_start(range.end() - ByteOffset(1)), //@todo - get token size from delimiter type
            false,
        )
    }
}

///
///
impl TokenCursor {
    pub fn new(tts: Vec<TokenTree>) -> Self {
        TokenCursor {
            frame: TokenCursorFrame {
                open_delim: FrameDelimiter::None,
                close_delim: FrameDelimiter::None,
                tts,
            },
            stack: Vec::new(),
        }
    }

    pub fn advance(&mut self) -> TokenAndSpan {
        loop {
            let tree = if let Some(tok) = self.frame.open_delim.consume() {
                return tok;
            } else if !self.frame.tts.is_empty() {
                self.frame.tts.remove(0)
            } else if let Some(tok) = self.frame.close_delim.consume() {
                return tok;
            } else if let Some(frame) = self.stack.pop() {
                self.frame = frame;
                continue;
            } else {
                return TokenAndSpan(Token::Eof, ByteSpan::default());
            };

            match tree {
                TokenTree::Token(sp, tok) => return TokenAndSpan(tok, sp),
                TokenTree::Delimited(sp, ty, tts) => {
                    let frame = TokenCursorFrame {
                        open_delim: FrameDelimiter::from_open_delimiter(ty, sp),
                        close_delim: FrameDelimiter::from_close_delimiter(ty, sp),
                        tts,
                    };

                    self.stack.push(mem::replace(&mut self.frame, frame));
                }
            }
        }
    }
}

impl Iterator for TokenCursor {
    type Item = TokenAndSpan;

    fn next(&mut self) -> Option<TokenAndSpan> {
        let next = self.advance();

        if let TokenAndSpan(Token::Eof, ..) = next {
            None
        } else {
            Some(next)
        }
    }
}
