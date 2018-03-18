//! AST types for the `secsp` parser.  The root component of all ASTs is a `Statement`.
//!

use super::codemap::Span;

macro_rules! impl_spanned {
    ($name: ident) => {
        impl Spanned for $name {
            fn span(&self) -> Span {
                self.span
            }
        }
    };
}

/// A unique 64 bit identifier for `Node`s in the syntax tree.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeId(pub u64);

pub trait NodeVisitor {
    type Result;

    fn visit_binary_op_expr(
        &mut self,
        lhs: &ExpressionNode,
        operator: &BinOp,
        rhs: &ExpressionNode,
    ) -> Self::Result;

    fn visit_category_range_expr(&mut self, lo: &Ident, hi: &Ident) -> Self::Result;
    fn visit_context_expr(
        &mut self,
        user: &Ident,
        role: &Ident,
        ty: &Ident,
        level_range: Option<&ExpressionNode>,
    ) -> Self::Result;
    fn visit_level_expr(
        &mut self,
        sensitivity: &Ident,
        categories: &ExpressionNode,
    ) -> Self::Result;
    fn visit_level_range_expr(&mut self, lo: &ExpressionNode, hi: &ExpressionNode) -> Self::Result;
    fn visit_unary_op_expr(&mut self, op: &UnaryOp, operand: &ExpressionNode) -> Self::Result;
    fn visit_var_expr(&mut self, span: &Ident) -> Self::Result;
    fn visit_varlist_expr(&mut self, vars: &[Ident]) -> Self::Result;

    fn visit_class_decl(
        &mut self,
        ty: &ClassType,
        id: &Ident,
        access_vectors: &[Ident],
    ) -> Self::Result;

    fn visit_conditional(
        &mut self,
        ty: &ConditionalType,
        expr: &ExpressionNode,
        body: &StatementNodeList,
        else_ifs: &[(ExpressionNode, Vec<StatementNode>)],
        else_blk: Option<&StatementNodeList>,
    ) -> Self::Result;

    fn visit_container_decl(
        &mut self,
        ty: &ContainerType,
        id: &Ident,
        body: &StatementNodeList,
    ) -> Self::Result;
    fn visit_macro_decl(
        &mut self,
        id: &Ident,
        params: &[MacroParameter],
        body: &StatementNodeList,
    ) -> Self::Result;
    fn visit_symbol_decl(
        &mut self,
        ty: &SymbolType,
        id: &Ident,
        value: Option<&ExpressionNode>,
    ) -> Self::Result;
    fn visit_te_rule(
        &mut self,
        ty: &TeRuleType,
        source: &ExpressionNode,
        target: &ExpressionNode,
        perms: &ExpressionNode,
    ) -> Self::Result;
}

pub trait Spanned {
    /// Get the codemap information representing this `Node`.
    fn span(&self) -> Span;
}

/// A trait representing a single unique node in the syntax tree.
pub trait Node: Sized + Spanned {
    /// Make the given `NodeVisitor` visit this node and return a `V::Result`.
    fn accept<V: NodeVisitor>(&self, visitor: &mut V) -> V::Result;

    /// Get this `Node`s unique identifier.
    fn node_id(&self) -> NodeId;
}

pub struct Module {
    name: Option<String>,
    body: Vec<StatementNode>,
    span: Span,
}

impl Module {
    pub fn new(name: Option<String>, body: Vec<StatementNode>, span: Span) -> Self {
        Module { name, body, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContainerType {
    /// A block container type.  Optionally abstract, with a list of 0 or more parent blocks.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// abstract block my_block {}
    /// ```
    ///
    /// ```csp,ignore
    /// block extends my_block, another_block {}
    /// ```
    Block(bool),

    /// An optional container type that can cause a build error and still allow the policy build to pass.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// optional my_optional_block { allow my_non_existent_type my_other_type file : read; }
    /// ```
    Optional,

    /// An extension container type that allows adding new policy within the context of an existing `Block` `ContainerType`.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// block my_block {}
    /// ```
    ///
    /// ```csp,ignore
    /// in my_block { /* add additional policy */ }
    /// ```
    Extends,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ClassType {
    /// A concrete definition of a security class which optionally inherits from a `Common` `ClassType`.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// class my_security_class extends my_base_security_class {}
    /// ```
    Class,

    /// An abstract definition of a security class, used only for sharing permissions between concrete `Class` `ClassTypes`.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// common my_base_security_class {}
    /// ```
    Common,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolType {
    /// A security type symbol.
    ///
    /// Example:
    /// ```csp,ignore
    /// type my_type;
    /// ```
    Type,

    /// A bitmap of `Type` symbols.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// type_attribute my_types;
    /// my_types |= (type) my_type;
    /// ```
    TypeAttribute,

    /// A security role symbol.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// role my_role;
    /// ```
    Role,

    /// A bitmap of `Role` symbols.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// role_attribute my_roles;
    /// my_roles |= (role) my_role;
    /// ```
    RoleAttribute,

    /// A security user symbol.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// user my_user;
    /// ```
    User,

    /// A bitmap of security users.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// user_attribute my_users;
    /// my_users |= (user) my_user;
    /// ```
    UserAttribute,

    /// A definition of a named security context, containing a `User`, `Role`, `Type`, and optional `LevelRange`.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// context my_context = my_user:my_role:my_type;
    /// context my_mls_context = my_user:my_role_:my_type:my_level_range;
    /// ```
    Context,

    /// A sensitivity level symbol.
    Sensitivity,

    /// A range of two sensitivity levels and category sets.
    LevelRange,

    /// A security category symbol.
    Category,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TeRuleType {
    /// Denotes that the set of access-vectors for the given source and target on a type-enforcement rule should be allowed.
    Allow,

    /// Denotes that the set of access-vectors for the given source and target on a type-enforcement rule should be treated
    /// as invalid by the policy compiler.
    NeverAllow,

    /// Denotes that the set of access-vectors for the given source and target on a type-enforcement rule should be sent to the
    /// audit subsystem whenever the security server allows them.
    AuditAllow,

    /// Denotes that the set of access-vectors for the given source and target on a type-enforcement rule should not be sent
    /// to the audit subsystem when they are denied.
    DontAudit,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConditionalType {
    /// A conditional that can be tuned at runtime and does not require a rebuild from policy sources.
    Boolean,

    /// A conditional that can only be tuned at policy build time.
    Tunable,
}

/// An AST node representing a `StatementKind` with a unique ID and codemap information.
#[derive(Clone, Debug, PartialEq)]
pub struct StatementNode {
    pub node_id: NodeId,
    pub kind: Box<StatementKind>,
    pub span: Span,
}

impl_spanned!(StatementNode);

pub type StatementNodeList = [StatementNode];

/// Representations of different types of `StatementNode`s that can occur in policy sources.
#[derive(Clone, Debug, PartialEq)]
pub enum StatementKind {
    /// A declaration of an abstract or concrete security class and its set of permissions.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// class file {
    ///    read,
    ///    write,
    /// }
    /// ```
    ClassDeclaration(ClassType, Ident, Vec<Ident>),

    /// A declaration of a container with a body of `StatementNode`s.
    ContainerDeclaration(ContainerType, Ident, Vec<StatementNode>),

    /// A conditional statement, representing a block of runtime or build time conditional policy with 0 or more else-ifs and an optional
    /// else block.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// booleanif my_runtime_boolean {
    ///    /* ... */
    /// }
    /// ```
    ///
    /// ```csp,ignore
    /// tunableif my_build_time_tunable {
    ///    /* ... */
    /// } else if my_other_tunable {
    /// } else {
    /// }
    /// ```
    Conditional(
        ConditionalType,
        ExpressionNode,
        Vec<StatementNode>,
        Vec<(ExpressionNode, Vec<StatementNode>)>,
        Option<Vec<StatementNode>>,
    ),

    /// A declaration of a named macro with an identifier, list of parameters, and body.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// macro my_macro(type ty) {
    ///    // ....
    /// }
    /// ```
    MacroDeclaration(Ident, Vec<MacroParameter>, Vec<StatementNode>),

    /// A declaration of a named symbol with an optional initializer.
    ///
    /// Example:
    ///
    /// ```csp,ignore
    /// type my_type;
    /// ```
    ///
    /// ```csp,ignore
    /// context c = user:role:type;
    /// ```
    SymbolDeclaration(SymbolType, Ident, Option<ExpressionNode>),

    /// A type-enforcement rule statement with a rule type, source expression, target expression, and access vector expression.
    TeRule(TeRuleType, ExpressionNode, ExpressionNode, ExpressionNode),
}

impl Node for StatementNode {
    /// Make the given `NodeVisitor` visit this node and return a `V::Result`.
    fn accept<V: NodeVisitor>(&self, visitor: &mut V) -> V::Result {
        use self::StatementKind::*;

        match *self.kind {
            ClassDeclaration(ref ty, ref id, ref perms) => visitor.visit_class_decl(ty, id, perms),
            ContainerDeclaration(ref ty, ref id, ref body) => {
                visitor.visit_container_decl(ty, id, &body[..])
            }
            Conditional(ref ty, ref expr, ref body, ref else_ifs, ref else_blk) => visitor
                .visit_conditional(
                    ty,
                    expr,
                    &body[..],
                    &else_ifs[..],
                    else_blk.as_ref().map(|x| &x[..]),
                ),
            MacroDeclaration(ref id, ref params, ref body) => {
                visitor.visit_macro_decl(id, params, &body[..])
            }
            SymbolDeclaration(ref ty, ref id, ref val) => {
                visitor.visit_symbol_decl(ty, id, val.as_ref())
            }
            TeRule(ref ty, ref source, ref target, ref perms) => {
                visitor.visit_te_rule(ty, source, target, perms)
            }
        }
    }

    /// Get this `Node`s unique identifier.
    fn node_id(&self) -> NodeId {
        self.node_id
    }
}

/// An AST node representing an `ExpressionKind` with a unique ID and coedmap information.
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionNode {
    /// A unique identifier for this node.
    pub node_id: NodeId,

    /// The underlying `ExpressionKind` this node is representing.
    pub kind: Box<ExpressionKind>,

    /// Location and codemap information from the original source.
    pub span: Span,
}

impl_spanned!(ExpressionNode);

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind {
    BinaryOp(ExpressionNode, BinOp, ExpressionNode),
    CategoryRange(Ident, Ident),
    Context(Ident, Ident, Ident, Option<ExpressionNode>),
    Level(Ident, ExpressionNode),
    LevelRange(ExpressionNode, ExpressionNode),
    UnaryOp(UnaryOp, ExpressionNode),
    Variable(Ident),
    VariableList(Vec<Ident>),
}

impl Node for ExpressionNode {
    /// Make the given `NodeVisitor` visit this node and return a `V::Result`.
    fn accept<V: NodeVisitor>(&self, visitor: &mut V) -> V::Result {
        use self::ExpressionKind::*;

        match *self.kind {
            BinaryOp(ref lhs, ref op, ref rhs) => visitor.visit_binary_op_expr(lhs, op, rhs),
            CategoryRange(ref lo, ref hi) => visitor.visit_category_range_expr(lo, hi),
            Context(ref user, ref role, ref ty, ref range) => {
                visitor.visit_context_expr(user, role, ty, range.as_ref())
            }
            Level(ref sensitivity, ref categories) => {
                visitor.visit_level_expr(sensitivity, categories)
            }
            LevelRange(ref lo, ref hi) => visitor.visit_level_range_expr(lo, hi),
            UnaryOp(ref op, ref operand) => visitor.visit_unary_op_expr(op, operand),
            Variable(ref var) => visitor.visit_var_expr(var),
            VariableList(ref varlist) => visitor.visit_varlist_expr(&varlist[..]),
        }
    }

    /// Get this `Node`s unique identifier.
    fn node_id(&self) -> NodeId {
        self.node_id
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Symbol<T: Sized> {
    value: T,
    span: Span,
}

impl<T> Spanned for Symbol<T> {
    fn span(&self) -> Span {
        self.span
    }
}

pub type Ident = Symbol<String>;
pub type BinOp = Symbol<BinOpKind>;
pub type UnaryOp = Symbol<UnaryOpKind>;

#[derive(Clone, Debug, PartialEq)]
pub enum BinOpKind {
    LogicalAnd,
    LogicalOr,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOpKind {
    UnaryNot,
    LogicalNot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MacroParameter {
    pub qualifier: SymbolType,
    pub name: Ident,
    pub span: Span,
}

impl_spanned!(MacroParameter);
