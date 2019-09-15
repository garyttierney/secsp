use std::marker::PhantomData;
use std::sync::Arc;

use rowan::GreenNode;

use secsp_parser::syntax::SyntaxNode;
use secsp_parser::ParseError;

use crate::ast::AstNode;
use crate::SourceFile;

use self::text_token_source::TextTokenSource;
use self::text_tree_sink::TextTreeSink;

#[cfg(test)]
mod tests;
mod text_token_source;
mod text_tree_sink;
mod tokenizer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parse<T: AstNode> {
    green: GreenNode,
    errors: Arc<Vec<ParseError>>,
    _ty: PhantomData<fn() -> T>,
}

impl<T: AstNode> Parse<T> {
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

pub fn parse_text<T>(text: T) -> Parse<SourceFile>
where
    T: AsRef<str>,
{
    let text = text.as_ref();
    let tokens = tokenizer::tokenize(text);
    let token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);
    secsp_parser::parse_file(&token_source, &mut tree_sink);

    let (green, errors) = tree_sink.finish();

    Parse {
        green,
        errors: Arc::new(errors),
        _ty: PhantomData,
    }
}
