use rowan::GreenNode;

use secsp_parser::ParseError;

use self::text_token_source::TextTokenSource;
use self::text_tree_sink::TextTreeSink;

#[cfg(test)]
mod tests;
mod text_token_source;
mod text_tree_sink;
mod tokenizer;

pub fn parse_text<T>(text: T) -> (GreenNode, Vec<ParseError>)
where
    T: AsRef<str>,
{
    let text = text.as_ref();
    let tokens = tokenizer::tokenize(text);
    let token_source = TextTokenSource::new(text, &tokens);
    let mut tree_sink = TextTreeSink::new(text, &tokens);
    secsp_parser::parse_file(&token_source, &mut tree_sink);

    tree_sink.finish()
}
