use rowan::{GreenNode, GreenNodeBuilder, SmolStr, SyntaxKind};
use text_unit::{TextRange, TextUnit};

use secsp_parser::{ParseError, TreeSink};

use crate::token::Token;
use std::mem;

enum State {
    PendingStart,
    Normal,
    PendingFinish,
}

pub struct TextTreeSink<'t> {
    text: &'t str,
    tokens: &'t [Token],
    token_pos: usize,
    text_pos: TextUnit,
    builder: GreenNodeBuilder,
    errors: Vec<ParseError>,
    state: State,
}

impl<'t> TextTreeSink<'t> {
    pub fn new(text: &'t str, tokens: &'t [Token]) -> Self {
        TextTreeSink {
            text,
            tokens,
            token_pos: 0,
            text_pos: TextUnit::from(0),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
            state: State::PendingStart,
        }
    }

    pub fn finish(mut self) -> (GreenNode, Vec<ParseError>) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingFinish => self.builder.finish_node(),
            State::Normal | State::PendingStart => (),
        };
        (self.builder.finish(), self.errors)
    }

    fn eat_trivias(&mut self) {
        while let Some(&token) = self.tokens.get(self.token_pos) {
            if !token.is_trivia() {
                break;
            }
            self.do_token(token.kind(), token.len());
        }
    }

    fn eat_n_trivias(&mut self, n: usize) {
        for _ in 0..n {
            let token = self.tokens[self.token_pos];
            assert!(token.is_trivia());
            self.do_token(token.kind(), token.len());
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, len: TextUnit) {
        let range = TextRange::offset_len(self.text_pos, len);
        let text: SmolStr = self.text[range].into();
        self.text_pos += len;
        self.token_pos += 1;
        self.builder.token(kind, text);
    }
}

impl<'t> TreeSink for TextTreeSink<'t> {
    fn error(&mut self, error: ParseError) {
        unimplemented!()
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => {
                self.builder.start_node(kind);
                // No need to attach trivias to previous node: there is no
                // previous node.
                return;
            }
            State::PendingFinish => self.builder.finish_node(),
            State::Normal => (),
        }

        let n_trivias = self.tokens[self.token_pos..]
            .iter()
            .take_while(|it| it.is_trivia())
            .count();
        let leading_trivias = &self.tokens[self.token_pos..self.token_pos + n_trivias];
        let mut trivia_end =
            self.text_pos + leading_trivias.iter().map(|it| it.len()).sum::<TextUnit>();

        let n_attached_trivias = {
            let leading_trivias = leading_trivias
                .iter()
                .rev()
                .map(|it| {
                    let next_end = trivia_end - it.len();
                    let range = TextRange::from_to(next_end, trivia_end);
                    trivia_end = next_end;
                    (it.kind(), &self.text[range])
                })
                .count();
            leading_trivias
        };
        self.eat_n_trivias(n_trivias - n_attached_trivias);
        self.builder.start_node(kind);
        self.eat_n_trivias(n_attached_trivias);
    }

    fn finish_node(&mut self) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.builder.finish_node(),
            State::Normal => (),
        }
    }

    fn token(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.builder.finish_node(),
            State::Normal => (),
        }
        self.eat_trivias();
        let tok = self.tokens[self.token_pos];
        let len = TextUnit::from_usize(tok.range().end - tok.range().start);

        self.do_token(kind, len);
    }
}
