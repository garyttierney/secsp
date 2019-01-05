use rowan::GreenNode;
use rowan::GreenNodeBuilder;
use rowan::Types;
use smol_str::SmolStr;

use crate::ast::{SyntaxError, SyntaxKind};
use crate::parser::event::EventSink;
use crate::parser::input::SyntaxKindBase;

pub struct SyntaxTreeBuilder<K: SyntaxKindBase, T: Types<Kind = K>> {
    inner: rowan::GreenNodeBuilder<T>,
    errors: Vec<SyntaxError>,
}

impl<K: SyntaxKindBase, T: Types<Kind = K>> SyntaxTreeBuilder<K, T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K: SyntaxKindBase, T: Types<Kind = K>> Default for SyntaxTreeBuilder<K, T> {
    fn default() -> Self {
        SyntaxTreeBuilder {
            inner: GreenNodeBuilder::new(),
            errors: vec![],
        }
    }
}

impl<K: SyntaxKindBase, T: Types<Kind = K>> EventSink<K> for SyntaxTreeBuilder<K, T> {
    type Output = (GreenNode<T>, Vec<SyntaxError>);

    fn leaf(&mut self, kind: K, text: SmolStr) {
        self.inner.leaf(kind, text);
    }

    fn begin(&mut self, kind: K) {
        self.inner.start_internal(kind);
    }

    fn end(&mut self) {
        self.inner.finish_internal();
    }

    fn finish(self) -> Self::Output {
        (self.inner.finish(), self.errors)
    }
}
