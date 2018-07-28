use codespan::{ByteOffset, FileMap, RawOffset};
use std::str::CharIndices;

use crate::lex::ByteIndex;
use crate::lex::Span;

pub type TextRange = Span<ByteIndex>;

// A UTF-8 text reader that returns indexed strings/characters containing starting and ending byte
// positions.
pub struct TextReader<'input> {
    file_map: &'input FileMap,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
}

/// A character associated with a `TextPos` representing it's byte offsets in the input.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IndexedChar(pub TextRange, pub char);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IndexedString<'a>(pub TextRange, pub &'a str);

impl IndexedChar {
    /// Get the byte range of this character in its source data.
    pub fn position(&self) -> TextRange {
        self.0
    }

    /// Get the character value.
    pub fn char(&self) -> char {
        self.1
    }
}

impl<'input> TextReader<'input> {
    /// Create a new TextReader from the input string.
    pub fn new(file_map: &'input FileMap) -> Self {
        let mut chars = file_map.src().char_indices();

        TextReader {
            file_map,
            lookahead: chars.next(),
            chars,
        }
    }

    /// Read the next character if any is available without advancing the reader
    /// position.
    ///
    /// ```
    /// use secsp_syntax::lex::{ByteIndex, IndexedChar, TextReader, TextRange};
    ///
    /// let mut reader = TextReader::new("test");
    /// let next = reader.peek();
    /// let expected = Some(IndexedChar(TextRange::new(ByteIndex(0), ByteIndex(0)), 't'));
    ///
    /// assert_eq!(next, expected);
    /// assert_eq!(reader.next(), expected);
    /// ```
    ///
    pub fn peek(&mut self) -> Option<IndexedChar> {
        self.lookahead.map(|(start, ch)| {
            let offset = ByteOffset(start as RawOffset);
            let index = self.file_map.span().start() + offset;
            let len = ByteOffset::from_char_utf8(ch);
            let sp = TextRange::new(index, index + len);

            IndexedChar(sp, ch)
        })
    }

    /// Extract a byte range of characters using this reader, returning
    /// a reference to a slice of the input data.
    ///
    /// ```
    /// use secsp_syntax::lex::{ByteIndex, TextReader, TextRange};
    ///
    /// let reader = TextReader::new("test");
    /// let text = reader.range(TextRange::new(ByteIndex(0), ByteIndex(3)));
    ///
    /// assert_eq!("test", text);
    /// ```
    pub fn range(&self, range: TextRange) -> &str {
        self.file_map.src_slice(range).expect("invalid range")
    }
}

impl<'input> Iterator for TextReader<'input> {
    type Item = IndexedChar;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.peek();
        self.lookahead = self.chars.next();

        next
    }
}
