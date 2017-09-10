//! AST types for the `secsp` parser.  The root component of all ASTs is a `Statement`.

pub type Identifier = String;

/// A type-specifier that can be converted to and from a string.
pub trait TypeSpecifier: Sized {
    /// Try and match `value` against a known type for this `TypeSpecifier`.
    fn from(value: &str) -> Option<Self>;

    fn to_cil(&self) -> &'static str;
}

/// Types of built-in containers.
#[derive(Clone, Debug, PartialEq)]
pub enum BlockType {
    /// A declaration of a namespace block.
    Block,

    /// An optional block, that will be omitted if it does not link.
    Optional,
}

impl TypeSpecifier for BlockType {
    fn from(value: &str) -> Option<Self> {
        use self::BlockType::*;

        let spec = match value {
            "block" => Block,
            "optional" => Optional,
            _ => return None,
        };

        Some(spec)
    }

    fn to_cil(&self) -> &'static str {
        use self::BlockType::*;

        match *self {
            Block => "block",
            Optional => "optional",
        }
    }
}

/// Built-in types for types in the symbol table.
#[derive(Clone, Debug, PartialEq)]
pub enum SymbolType {
    /// A security type symbol.
    Type,

    /// A bitmap of security types.
    TypeAttribute,

    /// A security role symbol.
    Role,

    /// A bitmap of security roles.
    RoleAttribute,

    /// A security user symbol.
    User,

    /// A bitmap of security users.
    UserAttribute,

    /// A collection of security attributes.
    Context,

    /// A sensitivity level symbol.
    Sensitivity,

    /// A range of two sensitivity levels and category sets.
    LevelRange,

    /// A security category symbol.
    Category,
}

impl TypeSpecifier for SymbolType {
    fn from(value: &str) -> Option<Self> {
        use self::SymbolType::*;

        let spec = match value {
            "type" => Type,
            "type_attribute" => TypeAttribute,
            "user" => User,
            "user_attribute" => UserAttribute,
            "role" => Role,
            "role_attribute" => RoleAttribute,
            "context" => Context,
            _ => return None,
        };

        Some(spec)
    }

    fn to_cil(&self) -> &'static str {
        use self::SymbolType::*;

        match *self {
            Type => "type",
            TypeAttribute => "typeattribute",
            User => "user",
            UserAttribute => "userattribute",
            Role => "role",
            RoleAttribute => "roleattribute",
            Context => "context",
            Sensitivity => "sensitivity",
            LevelRange => "levelrange",
            Category => "category",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AllowRuleType {
    Allow,
    NeverAllow,
    AuditAllow,
    DontAudit,
}

impl TypeSpecifier for AllowRuleType {
    fn from(value: &str) -> Option<Self> {
        use self::AllowRuleType::*;

        let spec = match value {
            "allow" => Allow,
            "auditallow" => AuditAllow,
            "dontaudit" => DontAudit,
            "neverallow" => NeverAllow,
            _ => return None,
        };

        Some(spec)
    }

    fn to_cil(&self) -> &'static str {
        use self::AllowRuleType::*;

        match *self {
            Allow => "allow",
            AuditAllow => "auditallow",
            DontAudit => "dontaudit",
            NeverAllow => "neverallow",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AccessVector {
    Permission(Expr),
    ClassAndPermissions(Expr, Expr),
}

/// Simple statement.
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    /// A declaration statement, declaring either a `Block`, `Symbol`, or `Macro`.
    Declaration(Declaration),
    MacroCall(Identifier, Vec<Expr>),
    IfElse {
        condition: Expr,
        then_block: Vec<Statement>,
        else_ifs: Vec<(Expr, Vec<Statement>)>,
        else_block: Option<Vec<Statement>>,
    },
    AccessVectorRule {
        rule_type: AllowRuleType,
        source: Box<Expr>,
        target: Box<Expr>,
        access_vector: Box<AccessVector>,
    },
    SetModifier {
        name: Identifier,
        cast: SymbolType,
        expr: Box<Expr>,
    },
}

/// A declaration statement.
#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    Block {
        is_abstract: bool,
        qualifier: BlockType,
        name: Identifier,
        statements: Vec<Statement>,
        extends: Option<Vec<Identifier>>,
    },
    Macro {
        name: Identifier,
        parameters: Vec<MacroParameter>,
        statements: Vec<Statement>,
    },
    Symbol {
        qualifier: SymbolType,
        name: Identifier,
        initializer: Option<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Variable(Identifier),
    VariableList(Vec<Identifier>),
    Level {
        sensitivity: Identifier,
        categories: Box<Expr>,
    },
    Context {
        user_id: Identifier,
        role_id: Identifier,
        type_id: Identifier,
        level_range: Option<Box<Expr>>,
    },
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    LevelRange(Box<Expr>, Box<Expr>),
    CategoryRange(Identifier, Identifier),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    ConditionalAnd,
    ConditionalOr,
    ConditionalXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    ConditionalNot,
    BitwiseNot,
}

impl Expr {
    pub fn var<S>(value: S) -> Expr
    where
        S: Into<Identifier>,
    {
        Expr::Variable(value.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContextExpr {
    pub user_id: Identifier,
    pub role_id: Identifier,
    pub type_id: Identifier,
    pub level_range: Option<Box<Expr>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LevelExpr {
    pub sensitivity: Identifier,
    pub categories: Box<Expr>,
}

/// A generic block statement containing an optional collection of other statements.
#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    /// If this block can be inherited from.  Only valid for `BlockType::Block`.
    pub is_abstract: bool,

    /// The type specifier that qualifies this block.
    pub qualifier: BlockType,

    /// The name of this block.
    pub name: Identifier,

    /// The list of statements contained in thsi block.
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Macro {
    pub name: Identifier,
    pub parameters: Vec<MacroParameter>,
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MacroParameter {
    pub qualifier: SymbolType,
    pub name: Identifier,
}