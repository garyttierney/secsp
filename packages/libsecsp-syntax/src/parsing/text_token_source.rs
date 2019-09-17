use secsp_parser::syntax::SyntaxKind;
use secsp_parser::TokenSource;

use crate::token::Token;

pub(crate) struct TextTokenSource<'t> {
    text: &'t str,
    tokens: Vec<Token>,
}

impl<'t> TextTokenSource<'t> {
    pub fn new(text: &'t str, tokens: &'t [Token]) -> Self {
        let non_trivia_tokens = tokens
            .iter()
            .filter(|tok| !tok.is_trivia())
            .cloned()
            .collect();

        TextTokenSource {
            text,
            tokens: non_trivia_tokens,
        }
    }
}

impl<'t> TokenSource for TextTokenSource<'t> {
    fn kind(&self, idx: usize) -> SyntaxKind {
        if idx >= self.tokens.len() {
            return SyntaxKind::TOK_EOF;
        }

        self.tokens[idx].kind()
    }

    fn text(&self, idx: usize) -> &str {
        if idx >= self.tokens.len() {
            return "";
        }

        let tok = &self.tokens[idx];
        let range = tok.range();

        &self.text[range]
    }
}
