use crate::ast::{AstNode, ItemOwner};
use crate::SourceFile;

pub(crate) fn parse_and_find<T: AstNode>(text: &str) -> T {
    SourceFile::parse(text)
        .tree()
        .items_of::<T>()
        .nth(0)
        .expect("No item of this type found")
}
