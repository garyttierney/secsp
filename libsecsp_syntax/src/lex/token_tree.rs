use crate::lex::token::DelimiterType;
use crate::lex::{ByteSpan, Token};

#[derive(Eq, PartialEq, Debug)]
pub enum TokenTree {
    Token(ByteSpan, Token),
    Delimited(ByteSpan, DelimiterType, Vec<TokenTree>),
}
