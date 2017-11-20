use ast::*;
use nom::{Err as NomErr, ErrorKind, IResult};
use nom::IResult::*;

/// Check if the given byte is valid in an identifier.
fn is_identifier(c: u8) -> bool {
    let ch = char::from(c);
    ch.is_alphanumeric() || ch == '_'
}

named!(identifier_raw <&[u8], &[u8]>, take_while1!(is_identifier));
named!(pub identifier <&[u8], Identifier>, map!(identifier_raw, |bytes: &[u8]| String::from_utf8(bytes.to_vec()).unwrap()));


named!(pub string_literal<&[u8], String>,
       map!(
           delimited!(
               tag!("\""),
               escaped!(is_not!("\n\"\\"), '\\', escape_code),
               tag!("\"")
            ),
            |bytes: &[u8]| String::from_utf8(bytes.to_vec()).unwrap()
       )
);

named!(escape_code<char>, one_of!("\'\"\\\n0123456789abfnrtuvxz"));

/// Match an `identifier` against a built-in type specifier, returning
/// an error if there is no match.
pub fn type_specifier<T: TypeSpecifier>(i: &[u8]) -> IResult<&[u8], T> {
    let (remaining, identifier) = try_parse!(i, identifier);
    let type_specifier = T::from(identifier.as_ref());

    if type_specifier.is_none() {
        return Error(NomErr::Code(ErrorKind::AlphaNumeric));
    }

    Done(remaining, type_specifier.unwrap())
}

named!(pub variable<&[u8], Expr>, map!(identifier, Expr::Variable));

#[cfg(test)]
mod tests {
    use super::*;
    use testing::parse;

    #[test]
    pub fn parse_type_specifier() {
        assert_eq!(
            parse::<SymbolType, _>("type", type_specifier),
            SymbolType::Type
        );
    }
}