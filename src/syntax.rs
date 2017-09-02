pub type Identifier = String;

/// A type-specifier that can be converted to and from a string.
pub trait TypeSpecifier: Sized {
    /// Try and match `value` against a known type for this `TypeSpecifier`.
    fn from(value: &str) -> Option<Self>;
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
}

/// Simple statement.
#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    /// A declaration statement, declaring either a `Block`, `Symbol`, or `Macro`.
    Declaration(Declaration),
}

/// A declaration statement.
#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    Block(Block),
    Symbol(SymbolType, Identifier, Option<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Variable(Identifier),
    Level(LevelExpr),
    Context(ContextExpr),
    LevelRange(Box<Expr>, Box<Expr>),
    CategoryRange(Identifier, Identifier),
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
