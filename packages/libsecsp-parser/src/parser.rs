pub(crate) use marker::CompletedMarker;

use crate::parser::event::Event;
use crate::syntax::{SyntaxKind, TokenKind};
use crate::TokenSource;

pub(crate) mod event;
mod marker;

pub(crate) struct Parser<'t> {
    token_source: &'t dyn TokenSource,
    token_pos: usize,
    events: Vec<Event>,
}

impl<'t> Parser<'t> {
    pub(super) fn new(token_source: &'t dyn TokenSource) -> Self {
        Parser {
            token_source,
            token_pos: 0,
            events: Vec::new(),
        }
    }

    pub(super) fn finish(self) -> Vec<Event> {
        self.events
    }

    /// Check if the parser is currently positioned at the [expected] type.
    pub fn at(&self, expected: TokenKind) -> bool {
        self.current() == expected.syntax_kind()
    }

    pub fn eat_keyword<K>(&mut self, kw: K) -> bool
    where
        K: AsRef<str> + Into<SyntaxKind>,
    {
        let at_kw = self.at_text(&kw);
        if at_kw {
            self.bump_as(kw.into());
        }

        at_kw
    }

    /// Check if the parser is currently positioned at the expected node with
    /// matching text.
    pub fn at_text<S>(&self, text: S) -> bool
    where
        S: AsRef<str>,
    {
        self.current_text() == text.as_ref()
    }

    /// Advance the position of the parser within the token stream and produce an event
    /// for the current leaf.
    pub fn bump(&mut self) {
        if self.current() == SyntaxKind::TOK_EOF {
            return;
        }

        self.events.push(Event::Leaf(self.nth_kind(0)));
        self.token_pos += 1;
    }

    /// Advance the position of the parser within the token stream and produce and event
    /// for the current leaf, but remap it to the given [kind].
    pub fn bump_as<K: Into<SyntaxKind>>(&mut self, kind: K) {
        self.events.push(Event::Leaf(kind.into()));
        self.token_pos += 1;
    }

    /// Get the type of token the parser is currently at.
    pub fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Get the text of the token the parser is currently at.
    pub fn current_text(&self) -> &'t str {
        self.nth_text(0)
    }

    /// Check if the parser is currently positioned at the [expected] type and consume the token,
    /// advancing the parsers position.
    pub fn eat(&mut self, expected: TokenKind) -> bool {
        if self.at(expected) {
            self.bump();
            return true;
        }

        false
    }

    /// Notify the parser that an error occurred at the given position with [text] as the error
    /// message.
    pub fn error<S>(&mut self, _text: S)
    where
        S: AsRef<str>,
    {
    }

    /// Check if the parser is currently positioned at the [expected] type, consuming it and
    /// emitting an error if the current token doesn't match what is expected.
    pub fn expect(&mut self, expected: TokenKind) {
        if !self.eat(expected) {
            self.error(format!("expected {:#?}", expected));
        }
    }

    /// Check if the parser is currently positioned at a token type that matches
    /// any of the expected [items], consuming the token and emitting an
    /// error if the current token doesn't match any of the inputs.
    pub fn expect_one_of(&mut self, items: Vec<SyntaxKind>) {
        let current_kind = self.nth_kind(0);

        if items.iter().any(|k| *k == current_kind) {
            self.bump();
        } else {
            self.error("expected one of (todo)".to_string());
        }
    }

    /// Create a new empty marker at the parsers current position.
    pub fn mark(&mut self) -> marker::Marker {
        self.events.push(Event::BeginMarker);
        marker::Marker::new(self.events.len() - 1)
    }

    /// Get the `nth` lookahead token type, offset from the parsers current position.
    pub fn nth(&self, offset: usize) -> SyntaxKind {
        self.token_source.kind(self.token_pos + offset)
    }

    /// Get the `nth` lookahead token type, offset from the parsers current position.
    pub fn nth_kind(&self, offset: usize) -> SyntaxKind {
        self.token_source.kind(self.token_pos + offset)
    }

    /// Get the `nth` lookahead token text, offset from the parsers current position.
    pub fn nth_text(&self, offset: usize) -> &'t str {
        self.token_source.text(self.token_pos + offset)
    }
}
