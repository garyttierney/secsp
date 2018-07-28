#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprContext {
    Context,
    Type,
    Role,
    User,
    Category,
    Class,
    Conditional,
    Any,
}
