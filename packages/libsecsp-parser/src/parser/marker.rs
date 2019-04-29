use drop_bomb::DropBomb;
use rowan::SyntaxKind;

use crate::parser::event::Event;
use crate::parser::Parser;
use crate::syntax::SyntaxKindClass;

pub(crate) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    pub fn new(pos: usize) -> Self {
        Marker {
            pos,
            bomb: DropBomb::new(
                "A marker must be completed or abandoned before it goes out of scope",
            ),
        }
    }

    pub fn abandon(mut self, parser: &mut Parser) {
        match &mut parser.events[self.pos] {
            evt @ Event::BeginMarker => *evt = Event::Tombstone,
            e => unreachable!("trying to abandon a {:#?} marker", e),
        };

        if self.pos == parser.events.len() - 1 {
            parser.events.pop();
        }

        self.bomb.defuse()
    }

    pub fn complete<K>(mut self, parser: &mut Parser, kind: K) -> CompletedMarker
    where
        K: SyntaxKindClass,
    {
        let rowan_kind = kind.into_syntax_kind();
        match parser.events[self.pos] {
            ref mut evt @ Event::BeginMarker => *evt = Event::Begin(rowan_kind, None),
            _ => unreachable!(),
        };

        parser.events.push(Event::End);
        self.bomb.defuse();

        CompletedMarker::new(rowan_kind, self.pos)
    }
}

pub(crate) struct CompletedMarker {
    kind: rowan::SyntaxKind,
    pos: usize,
}

impl CompletedMarker {
    pub fn new(kind: SyntaxKind, pos: usize) -> Self {
        CompletedMarker { kind, pos }
    }

    pub fn precede(self, p: &mut Parser) -> Marker {
        let m = p.mark();

        match p.events[self.pos] {
            Event::Begin(_, ref mut forward_parent) => {
                *forward_parent = Some(m.pos - self.pos);
            }
            _ => unreachable!(),
        }

        m
    }

    pub fn kind(&self) -> rowan::SyntaxKind {
        self.kind
    }
}
