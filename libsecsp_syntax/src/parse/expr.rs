#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprContext {
    Context,
    LevelRange,
    Type,
    Role,
    User,
    Category,
    Class,
    Conditional,
    NoSecLiteral,
    Any,
}

impl Default for ExprContext {
    fn default() -> Self {
        ExprContext::Any
    }
}
