use logos::Logos;

include!("syntax-generated.rs");

impl From<SyntaxKind> for rowan::cursor::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        rowan::cursor::SyntaxKind(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CspLang {}
impl rowan::Language for CspLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::cursor::SyntaxKind) -> Self::Kind {
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::cursor::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<CspLang>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<CspLang>;
pub type SyntaxToken = rowan::SyntaxToken<CspLang>;
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

#[derive(Logos, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenKind {
    /// A name identifier token, containing a reference to the original source data.
    #[regex = "[a-zA-Z_][a-zA-Z0-9_]*"]
    Name,
    /// A string literal token, containing a reference to the original source data.
    #[regex = "\"([^\"\\\\]|\\\\.)*\""]
    String,
    #[token = "if"]
    IfKw,
    #[token = "else"]
    ElseKw,
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
    Tombstone,
}

impl Into<SyntaxKind> for TokenKind {
    fn into(self) -> SyntaxKind {
        self.syntax_kind()
    }
}

impl TokenKind {
    pub fn syntax_kind(self) -> SyntaxKind {
        use self::{SyntaxKind::*, TokenKind::*};

        match self {
            Ampersand => TOK_AMPERSAND,
            Caret => TOK_CARET,
            CloseBrace => TOK_CLOSE_BRACE,
            CloseParenthesis => TOK_CLOSE_PARENTHESIS,
            Colon => TOK_COLON,
            Comma => TOK_COMMA,
            Dot => TOK_DOT,
            DotDot => TOK_DOT_DOT,
            DoubleAmpersand => TOK_DOUBLE_AMPERSAND,
            DoublePipe => TOK_DOUBLE_PIPE,
            ElseKw => TOK_ELSE_KW,
            Eof => TOK_EOF,
            Equals => TOK_EQUALS,
            Exclamation => TOK_EXCLAMATION,
            False => TOK_FALSE,
            Hyphen => TOK_HYPHEN,
            IfKw => TOK_IF_KW,
            Illegal => TOK_ILLEGAL,
            Integer => TOK_INTEGER,
            LineComment => TOK_LINE_COMMENT,
            Name => TOK_NAME,
            OpenBrace => TOK_OPEN_BRACE,
            OpenParenthesis => TOK_OPEN_PARENTHESIS,
            Pipe => TOK_PIPE,
            PipeEquals => TOK_PIPE_EQUALS,
            Semicolon => TOK_SEMICOLON,
            String => TOK_STRING,
            Tilde => TOK_TILDE,
            Tombstone => TOK_TOMBSTONE,
            True => TOK_TRUE,
            Whitespace => TOK_WHITESPACE,
        }
    }
}

impl KeywordKind {
    pub fn is_var_type(&self) -> bool {
        use self::KeywordKind::*;

        match self {
            Type | TypeAttribute | Role | RoleAttribute | User | UserAttribute | Sensitivity
            | Category | LevelRange | Context | ClassPermission => true,
            _ => false,
        }
    }
}
