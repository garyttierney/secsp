use std::mem;

use smol_str::SmolStr;
use text_unit::{TextRange, TextUnit};

use crate::ast::SyntaxKind;
use crate::parser::input::SyntaxKindBase;
use crate::parser::input::TokenBase;
use std::ops::Range;

/// An event sink for parse events.
pub trait EventSink<K: SyntaxKindBase> {
    /// The type that is outputted by this [EventSink] upon [finish]ing.
    type Output;

    /// Event handler that is called for every token in the input stream.
    fn leaf(&mut self, kind: K, text: SmolStr);

    /// Event handler that is called at the beginning of a compound syntax kind.
    fn begin(&mut self, kind: K);

    /// Event handler that is called when a compound syntax kind has been completed.
    fn end(&mut self);

    /// Finish consuming all events and emit collected output.
    fn finish(self) -> Self::Output;
}

#[derive(Debug)]
pub enum Event<K: SyntaxKindBase> {
    BeginMarker,
    Begin(K, Option<usize>),
    Leaf(K),
    End,
    Error,
    Tombstone,
}

/// The [EventProcessor] takes eve
pub struct EventProcessor<'a, K: SyntaxKindBase, T: TokenBase<K>, S: EventSink<K>> {
    events: &'a mut [Event<K>],
    sink: S,
    text: &'a str,
    text_pos: TextUnit,
    tokens: &'a [T],
    token_pos: usize,
}

impl<'a, K: SyntaxKindBase, T: TokenBase<K>, S: EventSink<K>> EventProcessor<'a, K, T, S> {
    pub fn new(text: &'a str, tokens: &'a [T], sink: S, events: &'a mut [Event<K>]) -> Self {
        EventProcessor {
            events,
            sink,
            text,
            text_pos: 0.into(),
            tokens,
            token_pos: 0,
        }
    }

    pub fn process(mut self) -> S::Output {
        let mut forward_parents = Vec::new();

        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Tombstone) {
                Event::Leaf(kind) => {
                    self.eat_trivia();
                    self.bump(kind);
                }
                Event::Begin(kind, forward_parent) => {
                    forward_parents.push(kind);
                    let mut parent_idx = idx;
                    let mut fp = forward_parent;

                    while let Some(fwd) = fp {
                        parent_idx += fwd;
                        fp = match mem::replace(&mut self.events[parent_idx], Event::Tombstone) {
                            Event::Begin(kind, forward_parent) => {
                                forward_parents.push(kind);
                                forward_parent
                            }
                            Event::Tombstone => None,
                            e => {
                                unreachable!("found unresolved {:#?} at position {}", e, parent_idx)
                            }
                        };
                    }

                    for kind in forward_parents.drain(..).rev() {
                        self.start(kind);
                    }
                }
                Event::End => {
                    self.eat_trivia();
                    self.sink.end();
                }
                _ => {}
            }
        }

        self.sink.finish()
    }

    fn bump(&mut self, expected: K) {
        let current = self.tokens[self.token_pos];
        self.leaf(expected, current.range());
    }

    fn eat_trivia(&mut self) {
        while let Some(tok) = self.tokens.get(self.token_pos) {
            if !tok.is_trivia() {
                break;
            }

            self.leaf(tok.kind(), tok.range());
        }
    }

    fn eat_n_trivia(&mut self, count: usize) {
        for _ in 0..count {
            let current = self.tokens[self.token_pos];
            assert!(current.is_trivia());

            self.leaf(current.kind(), current.range());
        }
    }

    fn leaf(&mut self, kind: K, range: Range<usize>) {
        let text = &self.text[range];

        self.sink.leaf(kind, text.into());
        self.token_pos += 1;
    }

    fn start(&mut self, kind: K) {
        if kind.is_root() {
            self.sink.begin(kind);
            return;
        }

        let n_trivias = self.tokens[self.token_pos..]
            .iter()
            .take_while(|it| it.kind().is_trivia())
            .count();

        let leading_trivias = &self.tokens[self.token_pos..self.token_pos + n_trivias];
        let n_attached_trivias = leading_trivias
            .iter()
            .enumerate()
            .filter_map(|(idx, trivia)| {
                let kind = trivia.kind();

                if kind.is_trivia() && !kind.is_whitespace() {
                    Some(idx)
                } else {
                    None
                }
            })
            .last()
            .unwrap_or(0);

        self.eat_n_trivia(n_trivias - n_attached_trivias);
        self.sink.begin(kind);
        self.eat_n_trivia(n_attached_trivias);
    }
}
