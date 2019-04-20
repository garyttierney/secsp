use std::marker::PhantomData;

use drop_bomb::DropBomb;
use rowan::SyntaxKind;

use crate::ast;
use crate::ast::{AstNode, TreeArc};
use crate::grammar;
use crate::lexer::tokenize;
use crate::parser::event::{Event, EventSink};
use crate::parser::input::ParserInput;
use crate::parser::input::TokenBase;
use crate::parser::syntax::{NodeKind, TokenKind};
use crate::token::Token;

mod builder;
mod event;
mod input;
pub mod syntax;

// TODO: This is largely inspired by the rust parser in rust-analyzer's ra_syntax crate.
//       Could the common code be extracted out into a generic parser, similar to IntelliJ's
//       PsiParser interface?

/// A recursive-descent parser that emits output as parse events denoting where syntax tree nodes
/// begin, allowing the parser and construction of the AST structure to evolve separately.
///
/// This parser is not aware of trivia tokens (e.g. doc comments, whitespace) and instead deals
/// solely with module structure. For details on reconciling whitespace and documentation with
/// AST nodes see the [EventProcessor] in the [:event] module.
pub struct Parser<'a, T: TokenBase> {
    events: Vec<Event>,
    input: ParserInput<'a, T>,
    pos: usize,
}

impl<'a, T: TokenBase> Parser<'a, T> {
    pub fn new(input: ParserInput<'a, T>) -> Self {
        Parser {
            events: vec![],
            input,
            pos: 0,
        }
    }

    /// Check if the parser is currently positioned at the [expected] type.
    pub fn at<E>(&self, expected: E) -> bool
    where
        E: Into<rowan::SyntaxKind>,
    {
        self.current() == expected.into()
    }

    pub fn eat_keyword<K>(&mut self, kw: K) -> bool
    where
        K: AsRef<str> + Into<rowan::SyntaxKind>,
    {
        let at_kw = self.at_text(&kw);
        if at_kw {
            self.bump_as(kw);
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
        if self.current().0 == TokenKind::Eof as u16 {
            return;
        }

        self.events.push(Event::Leaf(self.current()));
        self.pos += 1;
    }

    /// Advance the position of the parser within the token stream and produce and event
    /// for the current leaf, but remap it to the given [kind].
    pub fn bump_as<F>(&mut self, kind: F)
    where
        F: Into<rowan::SyntaxKind>,
    {
        self.events.push(Event::Leaf(kind.into()));
        self.pos += 1;
    }

    /// Get the type of token the parser is currently at.
    pub fn current(&self) -> rowan::SyntaxKind {
        self.nth(0)
    }

    /// Get the text of the token the parser is currently at.
    pub fn current_text(&self) -> &'a str {
        self.nth_text(0)
    }

    /// Check if the parser is currently positioned at the [expected] type and consume the token,
    /// advancing the parsers position.
    pub fn eat<E>(&mut self, expected: E) -> bool
    where
        E: Into<rowan::SyntaxKind>,
    {
        if self.at(expected) {
            self.bump();
            return true;
        }

        return false;
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
    pub fn expect<E>(&mut self, expected: E)
    where
        E: Into<rowan::SyntaxKind>,
    {
        let kind = expected.into();

        if !self.eat(kind) {
            self.error(format!("expected {:#?}", kind));
        }
    }

    /// Check if the parser is currently positioned at a token type that matches
    /// any of the expected [items], consuming the token and emitting an
    /// error if the current token doesn't match any of the inputs.
    pub fn expect_one_of<E>(&mut self, items: Vec<E>)
    where
        E: Into<rowan::SyntaxKind>,
    {
        let current_kind = self.current();
        let kinds: Vec<SyntaxKind> = items.into_iter().map(|item| item.into()).collect();

        if kinds.iter().any(|k| *k == current_kind) {
            self.bump();
        } else {
            self.error(format!("expected one of (todo)"));
        }
    }

    /// Check if the parser is currently positioned at a token has text matching
    /// any of the expected [items], consuming the token and emitting an error
    /// if the current token text doesn't match any of the inputs.
    pub fn expect_one_of_str<S: Into<String>>(&mut self, items: Vec<S>) {
        let current_text = self.current_text();
        let strs: Vec<String> = items.into_iter().map(|item| item.into()).collect();

        if strs.iter().any(|str| *str == current_text) {
            self.bump();
        } else {
            self.error(format!("expected one of (todo)"));
        }
    }

    /// Create a new empty marker at the parsers current position.
    pub fn mark(&mut self) -> Marker<T> {
        self.events.push(Event::BeginMarker);
        Marker::new(self.events.len() - 1)
    }

    /// Get the `nth` lookahead token type, offset from the parsers current position.
    pub fn nth(&self, offset: usize) -> rowan::SyntaxKind {
        self.input.kind(self.pos + offset)
    }

    /// Get the `nth` lookahead token text, offset from the parsers current position.
    pub fn nth_text(&self, offset: usize) -> &'a str {
        self.input.text(self.pos + offset)
    }
}

pub struct Marker<T: TokenBase> {
    pos: usize,
    bomb: DropBomb,
    _phantom_token: PhantomData<T>,
}

impl<T: TokenBase> Marker<T> {
    pub fn new(pos: usize) -> Self {
        Marker {
            pos,
            bomb: DropBomb::new(
                "A marker must be completed or abandoned before it goes out of scope",
            ),
            _phantom_token: PhantomData,
        }
    }

    pub fn abandon(mut self, parser: &mut Parser<T>) {
        match &mut parser.events[self.pos] {
            evt @ Event::BeginMarker => *evt = Event::Tombstone,
            e => unreachable!("trying to abandon a {:#?} marker", e),
        };

        if self.pos == parser.events.len() - 1 {
            parser.events.pop();
        }

        self.bomb.defuse()
    }

    pub fn complete<K>(mut self, parser: &mut Parser<T>, kind: K) -> CompletedMarker<T>
    where
        K: Into<rowan::SyntaxKind>,
    {
        let rowan_kind = kind.into();
        match parser.events[self.pos] {
            ref mut evt @ Event::BeginMarker => *evt = Event::Begin(rowan_kind, None),
            _ => unreachable!(),
        };

        parser.events.push(Event::End);
        self.bomb.defuse();

        CompletedMarker::new(rowan_kind, self.pos)
    }
}

pub struct CompletedMarker<T: TokenBase> {
    kind: rowan::SyntaxKind,
    pos: usize,
    _phantom_token: PhantomData<T>,
}

impl<T: TokenBase> CompletedMarker<T> {
    pub fn new(kind: rowan::SyntaxKind, pos: usize) -> Self {
        CompletedMarker {
            kind,
            pos,
            _phantom_token: PhantomData,
        }
    }

    pub fn precede(self, p: &mut Parser<T>) -> Marker<T> {
        let m = p.mark();

        match p.events[self.pos] {
            Event::Begin(_, ref mut forward_parent) => {
                *forward_parent = Some(m.pos - self.pos);
            }
            _ => unreachable!(),
        }

        m
    }
}

pub fn parse_file(text: &str) -> TreeArc<ast::SourceFile> {
    let (node, errors) = parse(text, builder::SyntaxTreeBuilder::new(), grammar::root);
    let root = ast::SyntaxNode::new(node, Some(Box::new(errors)));

    assert_eq!(root.kind(), NodeKind::SourceFile);

    ast::SourceFile::cast(&root).unwrap().to_owned()
}

pub fn parse<S: EventSink>(text: &str, sink: S, parser: fn(&mut Parser<Token>)) -> S::Output {
    let tokens = tokenize(text);
    let input = ParserInput::new(text, &tokens);
    let mut parser_api = Parser::new(input);
    parser(&mut parser_api);

    event::EventProcessor::new(text, &tokens, sink, &mut parser_api.events).process()
}

pub type CspParser<'a> = Parser<'a, Token>;

impl<'a> CspParser<'a> {
    pub fn at_kw(&self) -> bool {
        self.at(TokenKind::Name) || self.at(TokenKind::IfKw) || self.at(TokenKind::ElseKw)
    }
}
