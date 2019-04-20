use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use smol_str::SmolStr;

use crate::ast::SyntaxError;
use crate::parser::event::EventSink;

pub struct SyntaxTreeBuilder {
    inner: rowan::GreenNodeBuilder,
    errors: Vec<SyntaxError>,
}

impl SyntaxTreeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for SyntaxTreeBuilder {
    fn default() -> Self {
        SyntaxTreeBuilder {
            inner: GreenNodeBuilder::new(),
            errors: vec![],
        }
    }
}

impl EventSink for SyntaxTreeBuilder {
    type Output = (GreenNode, Vec<SyntaxError>);

    fn leaf<K>(&mut self, kind: K, text: SmolStr)
    where
        K: Into<rowan::SyntaxKind>,
    {
        self.inner.token(kind.into(), text);
    }

    fn begin<K>(&mut self, kind: K)
    where
        K: Into<rowan::SyntaxKind>,
    {
        self.inner.start_node(kind.into());
    }

    fn end(&mut self) {
        self.inner.finish_node();
    }

    fn finish(self) -> Self::Output {
        (self.inner.finish(), self.errors)
    }
}
