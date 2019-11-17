//! The attribute set statement is responsible for modifying existing sets of permissions,
//! types, users, and roles. It can be used to append a simple list of names to the set, or
//! append the result of a complex expression.

use crate::parser::Parser;
use crate::syntax::KeywordKind;

pub(crate) fn attribute_set(p: &mut Parser, kind: KeywordKind) {
    let m = p.mark();
    p.bump();
}
