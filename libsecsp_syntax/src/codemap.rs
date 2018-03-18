use std::cmp;

/// A structure representing the start and end byte positions of a token
/// or span of code.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Span {
    /// The byte position of the starting character in this `Span`.
    pub start: usize,

    /// The byte position of the ending character in this `Span`.
    pub end: usize,
}

impl Span {
    /// Create a code span that ranges from `start` to `end`, inclusively.
    pub fn from(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    /// Create a code span at `pos` that occupies a single character.
    pub fn at(pos: usize) -> Self {
        Span {
            start: pos,
            end: pos,
        }
    }

    pub fn len(&self) -> usize {
        (self.end - self.start) + 1
    }

    pub fn join(&self, other: &Self) -> Self {
        Span {
            start: cmp::min(self.start, other.start),
            end: cmp::min(self.end, other.end),
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

/// A record of a source file, containing a mapping between line beginnings and
/// byte positions.
pub struct CodeMap<'a> {
    /// The bounds of this codemap.
    span: Span,
    source: &'a str,
    lines: Vec<usize>,
}

impl<'a> CodeMap<'a> {
    pub fn new(source: &'a str) -> Self {
        let start = 0 as usize;
        let end = source.len();
        let mut lines = vec![start];

        lines.extend(source.match_indices('\n').map(|(p, _)| p + 1));

        CodeMap {
            span: Span::from(start, end),
            source,
            lines,
        }
    }
}
