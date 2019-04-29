use crate::{ParseError, TreeSink};
use rowan::SyntaxKind;
use std::mem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Event {
    BeginMarker,
    Begin(SyntaxKind, Option<usize>),
    Leaf(SyntaxKind),
    End,
    Error,
    Tombstone,
}

pub fn process(sink: &mut dyn TreeSink, mut events: Vec<Event>) {
    let mut forward_parents = Vec::new();

    for i in 0..events.len() {
        match mem::replace(&mut events[i], Event::Tombstone) {
            Event::BeginMarker | Event::Tombstone => {}
            Event::Begin(kind, forward_parent) => {
                // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                // in the normal control flow, the parent-child relation: `A -> B -> C`,
                // while with the magic forward_parent, it writes: `C <- B <- A`.

                // append `A` into parents.
                forward_parents.push(kind);
                let mut idx = i;
                let mut fp = forward_parent;
                while let Some(fwd) = fp {
                    idx += fwd as usize;
                    // append `A`'s forward_parent `B`
                    fp = match mem::replace(&mut events[idx], Event::Tombstone) {
                        Event::Begin(kind, forward_parent) => forward_parent,
                        _ => unreachable!(),
                    };
                    // append `B`'s forward_parent `C` in the next stage.
                }

                for kind in forward_parents.drain(..).rev() {
                    sink.start_node(kind);
                }
            }
            Event::End => sink.finish_node(),
            Event::Leaf(kind) => {
                sink.token(kind);
            }
            Event::Error => sink.error(ParseError("no error message handling yet".to_string())),
        }
    }
}
