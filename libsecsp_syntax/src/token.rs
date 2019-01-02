use crate::ast::SyntaxKind;
use logos::Logos;
use smol_str::SmolStr;
use std::ops::Range;

#[derive(Logos, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    /// A name identifier token, containing a reference to the original source data.
    #[regex = "[a-zA-Z_][a-zA-Z0-9_]*"]
    Name,

    /// A string literal token, containing a reference to the original source data.
    #[regex = "\"([^\"\\\\]|\\\\.)*\""]
    String,

    #[token = "true"]
    True,

    #[token = "false"]
    False,

    #[regex = "0[xX][0-9a-fA-F]+"]
    #[regex = "[0-9]+"]
    Integer,

    /// An opening delimiter token of the given `DelimiterType`.
    #[token = "("]
    OpenParenthesis,

    /// A closing delimiter token of the given `DelimiterType`.
    #[token = ")"]
    CloseParenthesis,

    #[token = "{"]
    OpenBrace,

    #[token = "}"]
    CloseBrace,

    /// The semicolon token. Used to terminate statements.
    #[token = ";"]
    Semicolon,

    /// The period token.  Used to separate fully qualified names.
    #[token = "."]
    Dot,

    /// The double period token.  Used as the range operator.
    #[token = ".."]
    DotDot,

    /// The colon token.  Used as a security attribute delimiter.
    #[token = ":"]
    Colon,

    /// The hyphen token.  Used as a
    #[token = "-"]
    Hyphen,

    /// The comma token.  Used as a delimiter.
    #[token = ","]
    Comma,

    /// The equals token.  Used as an initializer and assignment token.
    #[token = "="]
    Equals,

    /// The bitwise binary AND operator: `&`.
    #[token = "&"]
    Ampersand,

    /// The bitwise binary OR operator: `|`.
    #[token = "|"]
    Pipe,

    /// The bitwise binary XOR operator: `^`.
    #[token = "^"]
    Caret,

    /// The bitwise unary NOT operator: `~`.
    #[token = "~"]
    Tilde,

    /// The logical AND operator: `&&`.
    #[token = "&&"]
    DoubleAmpersand,

    /// The logical OR operator: `||`.
    #[token = "||"]
    DoublePipe,

    /// The logical unary NOT operator: `!`.
    #[token = "!"]
    Exclamation,

    /// The pipe-equals operator, used for flipping on bits in bitsets.
    #[token = "|="]
    PipeEquals,

    /// A C-style line comment.
    #[regex = "//[^\n]*"]
    LineComment,

    /// Any whitespace token.
    #[regex = "\\s"]
    Whitespace,

    /// An unmatched token that produced an error.
    #[error]
    Illegal,

    /// A token indicating the end of file has been reached.
    #[end]
    Eof,
}

impl Into<SyntaxKind> for TokenType {
    fn into(self) -> SyntaxKind {
        SyntaxKind::Token(self)
    }
}

/// A single unit of output by the lexer.
#[derive(Copy, Clone, Debug)]
pub struct Token(pub(crate) TokenType, pub(crate) usize, pub(crate) usize);

impl Token {
    pub fn new(ty: TokenType, range: Range<usize>) -> Self {
        Token(ty, range.start, range.end)
    }

    pub fn ty(&self) -> TokenType {
        self.0
    }

    pub fn range(&self) -> Range<usize> {
        self.1..self.2
    }

    pub fn is_trivia(&self) -> bool {
        match self.0 {
            TokenType::LineComment | TokenType::Whitespace => true,
            _ => false,
        }
    }
}

impl Into<TokenType> for Token {
    fn into(self) -> TokenType {
        self.0
    }
}
