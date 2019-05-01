use std::convert::TryFrom;
use std::str::FromStr;

use logos::Logos;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use rowan::SyntaxKind;

const NODE_KIND_START: u16 = 1_000;
const KW_KIND_START: u16 = 10_000;

pub trait SyntaxKindClass:
    TryFrom<u16, Error = String> + Into<u16> + std::fmt::Debug + Copy + Eq + PartialEq
{
    fn into_kind(self) -> SyntaxKind {
        SyntaxKind(self.into())
    }

    fn from_kind(kind: SyntaxKind) -> Option<Self> {
        Self::try_from(kind.0).ok()
    }
}

#[derive(IntoPrimitive, TryFromPrimitive, Logos, Copy, Clone, Debug, PartialEq, Eq)]
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

impl SyntaxKindClass for TokenKind {}

#[repr(u16)]
#[derive(IntoPrimitive, TryFromPrimitive, Copy, Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    /// Syntax-tree marker for the a list of statements within `{ ... }`.
    Block = NODE_KIND_START,

    /// Syntax-tree marker for a named container.
    Container,

    /// Syntax-tree marker for a list of parent-containers in a container declaration.
    ExtendsList,

    /// Syntax-tree marker for a macro definition and its body.
    MacroDef,

    /// Syntax-tree marker for a macro call statement.
    MacroCall,

    /// Syntax-tree marker for the argument list of a macro call.
    MacroArgumentList,

    /// Syntax-tree marker for individual arguments within an argument list.
    MacroArgumentListItem,

    /// Syntax-tree marker for the parameter list of within the parenthesis of a macro definition.
    MacroParamList,

    /// Syntax-tree marker for an individual item in a macro definition's parameter list.
    MacroParamListItem,

    /// Syntax-tree marker for a variable declaration.
    Variable,

    // region NodeKind::Expr(...)
    BinaryExpr,

    CategoryRangeExpr,

    LevelExpr,

    LevelRangeExpr,

    ContextExpr,

    LiteralExpr,

    /// Syntax-tree marker for a sub-list expression that takes a subset of children from a named list.
    ListExpr,

    /// Syntax-tree marker for a reference expression that points to a path.
    PathExpr,

    ParenExpr,

    /// Syntax-tree marker for a unary expression with a token preceding another expression.
    PrefixExpr,

    // endregion
    // region NodeKind::Stmt(...)
    /// Syntax-tree marker for a conditional (if, else-if, else) statement.
    ConditionalStmt,

    // endregion
    /// Syntax-tree marker for the top level node in a files AST.
    SourceFile,

    ParseError,
}

impl SyntaxKindClass for NodeKind {}

macro_rules! enum_string_mapping {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        impl FromStr for $name {
            type Err = ();

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                let kind = match input {
                    $($val => $name::$variant,)*
                    _ => return Err(())
                };

                Ok(kind)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                match self {
                    $($name::$variant => $val),*
                }
            }
        }
    };
}

#[repr(u16)]
#[derive(IntoPrimitive, TryFromPrimitive, Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeywordKind {
    /// The `type` statement keyword, which declares a new named security type.
    Type = KW_KIND_START,
    /// The `type_attribute` statement keyword, which declares a new bitset of types.
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

impl SyntaxKindClass for KeywordKind {}

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
