use std::str::FromStr;

use logos::Logos;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    KW_ABSTRACT,
    KW_ALLOW,
    KW_AUDIT_ALLOW,
    KW_BLOCK,
    KW_CATEGORY,
    KW_DONT_AUDIT,
    KW_EXTENDS,
    KW_IN,
    KW_LEVEL_RANGE,
    KW_MACRO,
    KW_NEVER_ALLOW,
    KW_OPTIONAL,
    KW_ROLE,
    KW_ROLE_ATTRIBUTE,
    KW_SENSITIVITY,
    KW_TYPE,
    KW_TYPE_ATTRIBUTE,
    KW_USER,
    KW_USER_ATTRIBUTE,
    NODE_BINARY_EXPR,
    NODE_BLOCK,
    NODE_CATEGORY_RANGE_EXPR,
    NODE_CONDITIONAL_STMT,
    NODE_CONTAINER_DEF,
    NODE_CONTEXT_EXPR,
    NODE_EXTENDS_LIST,
    NODE_LEVEL_EXPR,
    NODE_LEVEL_RANGE_EXPR,
    NODE_LIST_EXPR,
    NODE_LITERAL_EXPR,
    NODE_MACRO_ARGUMENT_LIST,
    NODE_MACRO_ARGUMENT_LIST_ITEM,
    NODE_MACRO_CALL,
    NODE_MACRO_DEF,
    NODE_MACRO_PARAM_LIST,
    NODE_MACRO_PARAM_LIST_ITEM,
    NODE_NAME,
    NODE_PAREN_EXPR,
    NODE_PARSE_ERROR,
    NODE_PATH_EXPR,
    NODE_PREFIX_EXPR,
    NODE_SOURCE_FILE,
    NODE_TE_RULE,
    NODE_VARIABLE_DEF,
    TOK_AMPERSAND,
    TOK_CARET,
    TOK_CLOSE_BRACE,
    TOK_CLOSE_PARENTHESIS,
    TOK_COLON,
    TOK_COMMA,
    TOK_DOT,
    TOK_DOT_DOT,
    TOK_DOUBLE_AMPERSAND,
    TOK_DOUBLE_PIPE,
    TOK_ELSE_KW,
    TOK_EOF,
    TOK_EQUALS,
    TOK_EXCLAMATION,
    TOK_FALSE,
    TOK_HYPHEN,
    TOK_IF_KW,
    TOK_ILLEGAL,
    TOK_INTEGER,
    TOK_LINE_COMMENT,
    TOK_NAME,
    TOK_OPEN_BRACE,
    TOK_OPEN_PARENTHESIS,
    TOK_PIPE,
    TOK_PIPE_EQUALS,
    TOK_SEMICOLON,
    TOK_STRING,
    TOK_TILDE,
    TOK_TOMBSTONE,
    TOK_TRUE,
    TOK_WHITESPACE,
}

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

#[derive(
    IntoPrimitive, TryFromPrimitive, Logos, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u16)]
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

#[repr(u16)]
#[derive(
    IntoPrimitive, TryFromPrimitive, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[allow(non_camel_case_types)]
pub enum NodeKind {
    /// Syntax-tree marker for the a list of statements within `{ ... }`.
    BLOCK,
    /// Syntax-tree marker for a named container.
    CONTAINER_DEF,
    /// Syntax-tree marker for a list of parent-containers in a container declaration.
    EXTENDS_LIST,
    /// Syntax-tree marker for a macro definition and its body.
    MACRO_DEF,
    /// Syntax-tree marker for a macro call statement.
    MACRO_CALL,
    /// Syntax-tree marker for the argument list of a macro call.
    MACRO_ARGUMENT_LIST,
    /// Syntax-tree marker for individual arguments within an argument list.
    MACRO_ARGUMENT_LIST_ITEM,
    /// Syntax-tree marker for the parameter list of within the parenthesis of a macro definition.
    MACRO_PARAM_LIST,
    /// Syntax-tree marker for an individual item in a macro definition's parameter list.
    MACRO_PARAM_LIST_ITEM,
    /// Syntax-tree marker for a variable declaration.
    VARIABLE_DEF,

    // region SyntaxKind::NODE_Expr(...)
    BINARY_EXPR,
    CATEGORY_RANGE_EXPR,
    LEVEL_EXPR,
    LEVEL_RANGE_EXPR,
    CONTEXT_EXPR,
    LITERAL_EXPR,
    /// Syntax-tree marker for a type enforcement rule.
    TE_RULE,
    /// Syntax-tree marker for a sub-list expression that takes a subset of children from a named list.
    LIST_EXPR,
    /// Syntax-tree marker for a reference expression that points to a path.
    PATH_EXPR,
    PAREN_EXPR,
    /// Syntax-tree marker for a unary expression with a token preceding another expression.
    PREFIX_EXPR,

    // endregion
    // region SyntaxKind::NODE_Stmt(...)
    /// Syntax-tree marker for a conditional (if, else-if, else) statement.
    CONDITIONAL_STMT,

    // endregion
    /// Syntax-tree marker for the top level node in a files AST.
    SOURCE_FILE,
    PARSE_ERROR,
}

macro_rules! enum_string_mapping {
( enum $ name: ident {
$ ( $ variant: ident = $ val:expr), *,
}) => {
impl FromStr for $ name {
type Err = ();

fn from_str(input: & str) -> Result < Self, Self::Err > {
let kind = match input {
$ ( $ val => $ name::$ variant, ) *
_ => return Err(())
};

Ok(kind)
}
}

impl AsRef < str > for $ name {
fn as_ref( & self ) -> & str {
match self {
$ ( $ name::$ variant => $ val), *
}
}
}
};
}

#[repr(u16)]
#[derive(
    IntoPrimitive, TryFromPrimitive, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum KeywordKind {
    Type,
    TypeAttribute,
    /// The `role` statement keyword, which declares a new role for role-based access control.
    Role,
    /// The `role_attribute` statement keyword, which declares a new bitset of roles.
    RoleAttribute,
    /// The `user` statement keyword, which declares a new security identity.
    User,
    /// The `user_attribute` statement keyword, which declares a new bitset of users
    UserAttribute,
    /// The `optional` statement keyword, which declares a new optional container
    Optional,
    /// The `sensitivity` statement keyword, which declares a new multi-level security sensitivity level
    Sensitivity,
    /// The `category` statement keyword, which declares a new multi-level security compartment for compartmentalization
    Category,
    /// The `level_range` statement keyword, which declares a new low and high pair of sensitivities and category sets
    LevelRange,
    /// The `block` statement keyword, which declares a new namespace container
    Block,
    /// The `in` statement keyword, which extends an existing namespace container
    In,
    /// The `abstract` modifier, used to mark a `block` as abstract
    Abstract,
    /// The `extends` keyword, used to begin an inheritance list
    Extends,
    /// The `allow` statement keyword, which represents an allowed type-enforcement rule
    Allow,
    /// The `audit_allow` statement keyword, which represents a type-enforcement rule that logs when allowed
    AuditAllow,
    /// The `never_allow` statement keyword, which represents a build time type-enforcement check on `allow` rules
    NeverAllow,
    /// The `dont_audit` statement keyword, which represents a type-enforcement rule that prevents logging when denied
    DontAudit,
    /// The `macro` keyword, which defines a new macro.
    Macro,
}

impl Into<SyntaxKind> for KeywordKind {
    fn into(self) -> SyntaxKind {
        use self::{KeywordKind::*, SyntaxKind::*};

        match self {
            Type => KW_TYPE,
            TypeAttribute => KW_TYPE_ATTRIBUTE,
            Role => KW_ROLE,
            RoleAttribute => KW_ROLE_ATTRIBUTE,
            User => KW_USER,
            UserAttribute => KW_USER_ATTRIBUTE,
            Optional => KW_OPTIONAL,
            Sensitivity => KW_SENSITIVITY,
            Category => KW_CATEGORY,
            LevelRange => KW_LEVEL_RANGE,
            Block => KW_BLOCK,
            In => KW_IN,
            Abstract => KW_ABSTRACT,
            Extends => KW_EXTENDS,
            Allow => KW_ALLOW,
            AuditAllow => KW_AUDIT_ALLOW,
            NeverAllow => KW_NEVER_ALLOW,
            DontAudit => KW_DONT_AUDIT,
            Macro => KW_MACRO,
        }
    }
}

enum_string_mapping!(
    enum KeywordKind {
        Type = "type",
        TypeAttribute = "type_attribute",
        Role = "role",
        RoleAttribute = "role_attribute",
        User = "user",
        UserAttribute = "user_attribute",
        Optional = "optional",
        Sensitivity = "sensitivity",
        Category = "categoriy",
        LevelRange = "level_range",
        Block = "block",
        In = "in",
        Abstract = "abstract",
        Extends = "extends",
        Allow = "allow",
        AuditAllow = "audit_allow",
        NeverAllow = "never_allow",
        DontAudit = "dont_audit",
        Macro = "macro",
    }
);

impl KeywordKind {
    pub fn is_var_type(self) -> bool {
        use self::KeywordKind::*;

        match self {
            Type | TypeAttribute | Role | RoleAttribute | User | UserAttribute | Sensitivity
            | Category | LevelRange => true,
            _ => false,
        }
    }
}
