use nom::{Err as NomErr, ErrorKind, IResult, Needed, ParseTo, sp};
use nom::{alpha, is_alphanumeric, alphanumeric, digit, multispace, not_line_ending};
use nom::IResult::*;
use syntax;

named!(identifier_raw <&[u8], &[u8]>, take_while!(is_alphanumeric));
named!(identifier <&[u8], syntax::Identifier>, map!(identifier_raw, |bytes: &[u8]| String::from_utf8(bytes.to_vec()).unwrap()));

/// Match an `identifier` against a built-in type specifier, returning
/// an error if there is no match.
pub fn type_specifier<T: syntax::TypeSpecifier>(i: &[u8]) -> IResult<&[u8], T> {
    let (remaining, identifier) = try_parse!(i, identifier);
    let type_specifier = T::from(identifier.as_ref());

    if type_specifier.is_none() {
        return Error(NomErr::Code(ErrorKind::AlphaNumeric));
    }

    Done(remaining, type_specifier.unwrap())
}

/// Parse either a block or symbol declaration.
named!(pub declaration<&[u8], syntax::Declaration>,
  alt!(
    block_declaration | symbol_declaration 
  )
);

/// Parse a single named `Symbol` declaration.
named!(pub symbol_declaration<&[u8], syntax::Declaration>,
  ws!(do_parse!(
    qualifier: type_specifier >>
    name: identifier >>
    char!(';') >>

    (syntax::Declaration::Symbol(qualifier, name))
  ))
);

/// Parse a `block` or `optional` container, named by an `Identifer` and containing
/// a list of 0 or more `Statement`s.
named!(pub block_declaration<&[u8], syntax::Declaration>,
  ws!(do_parse!(
    is_abstract: opt!(tag!("abstract")) >>
    qualifier: type_specifier >>
    name: identifier >> 
    char!('{') >>
    char!('}') >>

    (syntax::Declaration::Block(syntax::Block {
      is_abstract: is_abstract.is_some(),
      qualifier,
      name,
      statements: vec![]
    }))
  ))
);

#[cfg(test)]
mod tests {

    use super::*;
    use syntax::*;

    /// Utility function that returns a completed Result with an empty
    /// input buffer and the given `value`.
    fn complete<'a, O>(value: O) -> IResult<&'a [u8], O> {
        Done(&b""[..], value)
    }

    #[test]
    pub fn parse_type_specifier() {
        assert_eq!(type_specifier(&b"type"[..]), complete(SymbolType::Type));
    }

    #[test]
    pub fn parse_block_decl() {
        assert_eq!(
            block_declaration(&b"block abc {}"[..]),
            complete(Declaration::Block(Block {
                is_abstract: false,
                qualifier: BlockType::Block,
                name: String::from("abc"),
                statements: vec![],
            }))
        );
    }

    #[test]
    pub fn parse_abstract_block_decl() {
        assert_eq!(
            block_declaration(&b"abstract block abc {}"[..]),
            complete(Declaration::Block(Block {
                is_abstract: true,
                qualifier: BlockType::Block,
                name: String::from("abc"),
                statements: vec![],
            }))
        );
    }

    #[test]
    pub fn parse_symbol_decl() {
        assert_eq!(
            symbol_declaration(&b"type mytype;"[..]),
            complete(Declaration::Symbol(
                SymbolType::Type,
                String::from("mytype"),
            ))
        );
    }
}