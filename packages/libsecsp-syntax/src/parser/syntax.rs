use std::convert::TryFrom;
use std::str::FromStr;

use logos::Logos;
use rowan::SyntaxKind;

const TOKEN_KIND_START: u16 = 0;
const NODE_KIND_START: u16 = 1_000;
const KW_KIND_START: u16 = 10_000;

// TODO: Generate these enums using some code generation tool.

trait InternalSyntaxKind: Sized {
    const START: u16;
    const END: u16;

    fn check_bounds(val: u16) -> Result<(), ()> {
        if val < Self::START || val > Self::END {
            return Err(());
        }

        Ok(())
    }
}

#[derive(Logos, Copy, Clone, Debug, PartialEq, Eq)]
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

    #[doc(hidden)]
    __LAST,
}

impl InternalSyntaxKind for TokenKind {
    const START: u16 = TokenKind::Name as u16;
    const END: u16 = TokenKind::__LAST as u16;
}

impl Into<SyntaxKind> for TokenKind {
    fn into(self) -> SyntaxKind {
        SyntaxKind(self as u16)
    }
}

impl TryFrom<SyntaxKind> for TokenKind {
    type Error = ();

    fn try_from(value: SyntaxKind) -> Result<Self, Self::Error> {
        Self::check_bounds(value.0)?;
        Ok(unsafe { std::mem::transmute(value.0) })
    }
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    #[doc(hidden)]
    __LAST,
}

impl InternalSyntaxKind for NodeKind {
    const START: u16 = NODE_KIND_START;
    const END: u16 = NodeKind::__LAST as u16;
}

impl Into<SyntaxKind> for NodeKind {
    fn into(self) -> SyntaxKind {
        SyntaxKind(self as u16)
    }
}

impl PartialEq<NodeKind> for SyntaxKind {
    fn eq(&self, other: &NodeKind) -> bool {
        self.0 == *other as u16
    }
}

impl TryFrom<SyntaxKind> for NodeKind {
    type Error = ();

    fn try_from(value: SyntaxKind) -> Result<Self, Self::Error> {
        Self::check_bounds(value.0)?;
        Ok(unsafe { std::mem::transmute(value.0) })
    }
}

#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeywordKind {
    TYPE = KW_KIND_START,
    //"The `type` statement keyword, which declares a new named security type.",
    TYPE_ATTRIBUTE,
    //"The `type_attribute` statement keyword, which declares a new bitset of types.",
    ROLE,
    //"The `role` statement keyword, which declares a new role for role-based access control.",
    ROLE_ATTRIBUTE,
    //"The `role_attribute` statement keyword, which declares a new bitset of roles.",
    USER,
    //"The `user` statement keyword, which declares a new security identity.",
    USER_ATTRIBUTE,
    //"The `user_attribute` statement keyword, which declares a new bitset of users",
    OPTIONAL,
    //"The `optional` statement keyword, which declares a new optional container",
    SENSITIVITY,
    //"The `sensitivity` statement keyword, which declares a new multi-level security sensitivity level",
    CATEGORY,
    //"The `category` statement keyword, which declares a new multi-level security compartment for compartmentalization",
    LEVEL_RANGE,
    //"The `level_range` statement keyword, which declares a new low and high pair of sensitivities and category sets",
    BLOCK,
    //"The `block` statement keyword, which declares a new namespace container",
    IN,
    //"The `in` statement keyword, which extends an existing namespace container",
    ABSTRACT,
    //"The `abstract` modifier, used to mark a `block` as abstract",
    EXTENDS,
    //"The `extends` keyword, used to begin an inheritance list",
    ALLOW,
    //"The `allow` statement keyword, which represents an allowed type-enforcement rule",
    AUDIT_ALLOW,
    //"The `audit_allow` statement keyword, which represents a type-enforcement rule that logs when allowed",
    NEVER_ALLOW,
    //"The `never_allow` statement keyword, which represents a build time type-enforcement check on `allow` rules",
    DONT_AUDIT,
    //"The `dont_audit` statement keyword, which represents a type-enforcement rule that prevents logging when denied",
    MACRO,
    #[doc(hidden)]
    __LAST,
}

impl FromStr for KeywordKind {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use self::KeywordKind::*;
        let ty = match input {
            "type" => TYPE,
            "type_attribute" => TYPE_ATTRIBUTE,
            "role" => ROLE,
            "role_attribute" => ROLE_ATTRIBUTE,
            "optional" => OPTIONAL,
            //"The `optional` statement keyword, which declares a new optional container",
            "sensitivity" => SENSITIVITY,
            //"The `sensitivity` statement keyword, which declares a new multi-level security sensitivity level",
            "categoriy" => CATEGORY,
            //"The `category` statement keyword, which declares a new multi-level security compartment for compartmentalization",
            "level_range" => LEVEL_RANGE,
            //"The `level_range` statement keyword, which declares a new low and high pair of sensitivities and category sets",
            "block" => BLOCK,
            //"The `block` statement keyword, which declares a new namespace container",
            "in" => IN,
            //"The `in` statement keyword, which extends an existing namespace container",
            "abstract" => ABSTRACT,
            //"The `abstract` modifier, used to mark a `block` as abstract",
            "extends" => EXTENDS,
            //"The `extends` keyword, used to begin an inheritance list",
            "allow" => ALLOW,
            //"The `allow` statement keyword, which represents an allowed type-enforcement rule",
            "audit_allow" => AUDIT_ALLOW,
            //"The `audit_allow` statement keyword, which represents a type-enforcement rule that logs when allowed",
            "never_allow" => NEVER_ALLOW,
            //"The `never_allow` statement keyword, which represents a build time type-enforcement check on `allow` rules",
            "dont_audit" => DONT_AUDIT,
            //"The `dont_audit` statement keyword, which represents a type-enforcement rule that prevents logging when denied",
            "macro" => MACRO,
            #[doc(hidden)]
            _ => return Err(()),
        };

        Ok(ty)
    }
}

impl AsRef<str> for KeywordKind {
    fn as_ref(&self) -> &str {
        use self::KeywordKind::*;

        match self {
            TYPE => "type",
            TYPE_ATTRIBUTE => "type_attribute",
            ROLE => "role",
            ROLE_ATTRIBUTE => "role_attribute",
            OPTIONAL => "optional",
            //"The `optional` statement keyword, which declares a new optional container",
            SENSITIVITY => "sensitivity",
            //"The `sensitivity` statement keyword, which declares a new multi-level security sensitivity level",
            CATEGORY => "categoriy",
            //"The `category` statement keyword, which declares a new multi-level security compartment for compartmentalization",
            LEVEL_RANGE => "level_range",
            //"The `level_range` statement keyword, which declares a new low and high pair of sensitivities and category sets",
            BLOCK => "block",
            //"The `block` statement keyword, which declares a new namespace container",
            IN => "in",
            //"The `in` statement keyword, which extends an existing namespace container",
            ABSTRACT => "abstract",
            //"The `abstract` modifier, used to mark a `block` as abstract",
            EXTENDS => "extends",
            //"The `extends` keyword, used to begin an inheritance list",
            ALLOW => "allow",
            //"The `allow` statement keyword, which represents an allowed type-enforcement rule",
            AUDIT_ALLOW => "audit_allow",
            //"The `audit_allow` statement keyword, which represents a type-enforcement rule that logs when allowed",
            NEVER_ALLOW => "never_allow",
            //"The `never_allow` statement keyword, which represents a build time type-enforcement check on `allow` rules",
            DONT_AUDIT => "dont_audit",
            //"The `dont_audit` statement keyword, which represents a type-enforcement rule that prevents logging when denied",
            MACRO => "macro",
            _ => unreachable!(),
        }
    }
}

impl InternalSyntaxKind for KeywordKind {
    const START: u16 = KW_KIND_START;
    const END: u16 = KeywordKind::__LAST as u16;
}

impl Into<SyntaxKind> for KeywordKind {
    fn into(self) -> SyntaxKind {
        SyntaxKind(self as u16)
    }
}

impl TryFrom<SyntaxKind> for KeywordKind {
    type Error = ();

    fn try_from(value: SyntaxKind) -> Result<Self, Self::Error> {
        Self::check_bounds(value.0)?;
        Ok(unsafe { std::mem::transmute(value.0) })
    }
}

impl KeywordKind {
    pub fn is_var_type(&self) -> bool {
        use self::KeywordKind::*;

        match *self {
            TYPE | TYPE_ATTRIBUTE | ROLE | ROLE_ATTRIBUTE | USER | USER_ATTRIBUTE | SENSITIVITY
            | CATEGORY | LEVEL_RANGE => true,
            _ => false,
        }
    }
}
