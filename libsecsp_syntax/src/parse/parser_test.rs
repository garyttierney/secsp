use crate::ast::Span;
use crate::ast::{
    ExpressionKind, ExpressionNode, Ident, NodeId, Path, StatementKind, StatementNode, Symbol,
    SymbolType,
};

pub(crate) fn symbol<S: Sized + PartialEq>(value: S) -> Symbol<S> {
    Symbol {
        span: Span::default(),
        value,
    }
}

pub(crate) fn decl<S: Into<String>>(
    kind: SymbolType,
    name: S,
    initializer: Option<ExpressionNode>,
) -> StatementNode {
    stmt(StatementKind::SymbolDeclaration(
        kind,
        symbol(name.into()),
        initializer,
    ))
}

pub(crate) fn stmt(kind: StatementKind) -> StatementNode {
    StatementNode {
        node_id: NodeId(0),
        span: Span::default(),
        kind: Box::from(kind),
    }
}

pub(crate) fn expr(kind: ExpressionKind) -> ExpressionNode {
    ExpressionNode {
        node_id: NodeId(0),
        span: Span::default(),
        kind: Box::from(kind),
    }
}

pub(crate) fn variable<S: Into<String>>(name: S) -> ExpressionNode {
    let val = Ident::new(name.into(), Span::default());

    ExpressionNode {
        kind: Box::from(ExpressionKind::Variable(Path::from_ident(val))),
        node_id: NodeId(0),
        span: Span::default(),
    }
}
