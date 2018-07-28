#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// An interned string identified by a unique id.
pub struct Symbol(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A structural code delimiter that can occur in source code.
pub enum DelimiterType {
    /// A curly brace delimiter.  i.e., '{' or '}'.
    Brace,

    /// A parenthesis delimiter.  i.e., '(' or ')'.
    Parenthesis,
}

/// A single lexical unit in the CSP grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    /// A name identifier token, containing a reference to the original source data.
    Name(String),

    /// A string literal token, containing a reference to the original source data.
    Literal(String),

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

    /// The comma token.  Used as a delimiter.
    Comma,

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

    /// A token indicating the end of file has been reached.
    Eof,
}
