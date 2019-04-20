use std::mem;
use std::ops::Range;

use rowan::SyntaxKind;
use smol_str::SmolStr;
use text_unit::TextUnit;

use crate::parser::input::SyntaxKindBase;
use crate::parser::input::TokenBase;

/// An event sink for parse events.
pub trait EventSink {
    /// The type that is outputted by this [EventSink] upon [finish]ing.
    type Output;

    /// Event handler that is called for every token in the input stream.
    fn leaf<K>(&mut self, kind: K, text: SmolStr)
    where
        K: Into<rowan::SyntaxKind>;

    /// Event handler that is called at the beginning of a compound syntax kind.
    fn begin<K>(&mut self, kind: K)
    where
        K: Into<rowan::SyntaxKind>;

    /// Event handler that is called when a compound syntax kind has been completed.
    fn end(&mut self);

    /// Finish consuming all events and emit collected output.
    fn finish(self) -> Self::Output;
}

#[derive(Debug)]
pub enum Event {
    BeginMarker,
    Begin(SyntaxKind, Option<usize>),
    Leaf(SyntaxKind),
    Trivia(SyntaxKind),
    Whitespace(SyntaxKind),
    End,
    Error,
    Tombstone,
}

/// The [EventProcessor] takes eve
pub struct EventProcessor<'a, T: TokenBase, S: EventSink> {
    events: &'a mut [Event],
    sink: S,
    text: &'a str,
    text_pos: TextUnit,
    tokens: &'a [T],
    token_pos: usize,
    started: bool,
}

impl<'a, T: TokenBase, S: EventSink> EventProcessor<'a, T, S> {
    pub fn new(text: &'a str, tokens: &'a [T], sink: S, events: &'a mut [Event]) -> Self {
        EventProcessor {
            events,
            sink,
            text,
            text_pos: 0.into(),
            tokens,
            token_pos: 0,
            started: false,
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

    fn bump<K>(&mut self, expected: K)
    where
        K: Into<SyntaxKind>,
    {
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

    fn leaf<K>(&mut self, kind: K, range: Range<usize>)
    where
        K: Into<SyntaxKind>,
    {
        let text = &self.text[range];

        self.sink.leaf(kind, text.into());
        self.token_pos += 1;
    }

    fn start<K>(&mut self, kind: K)
    where
        K: Into<SyntaxKind>,
    {
        if !self.started {
            self.started = true;
            self.sink.begin(kind);
            return;
        }

        let n_trivias = self.tokens[self.token_pos..]
            .iter()
            .take_while(|it| it.is_trivia())
            .count();

        let leading_trivias = &self.tokens[self.token_pos..self.token_pos + n_trivias];
        let n_attached_trivias = leading_trivias
            .iter()
            .enumerate()
            .filter_map(|(idx, tok)| {
                let _kind = tok.kind();

                if tok.is_trivia() && !tok.is_whitespace() {
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
