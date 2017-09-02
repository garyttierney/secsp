use nom::{Err as NomErr, ErrorKind, IResult};
use nom::IResult::*;
use syntax;

/// Check if the given byte is valid in an identifier.
fn is_identifier(c: u8) -> bool {
    let ch = char::from(c);
    ch.is_alphanumeric() || ch == '_'
}

named!(identifier_raw <&[u8], &[u8]>, take_while1!(is_identifier));
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

/// Parse a declaration as a statement.
named!(pub statement<&[u8], syntax::Statement>,
  alt!(
    map!(declaration, |decl| syntax::Statement::Declaration(decl))
  )
);

/// Parse either a block or symbol declaration.
named!(pub declaration<&[u8], syntax::Declaration>,
  alt!(
    map!(block_declaration, syntax::Declaration::Block) | symbol_declaration
  )
);

named!(pub expr<&[u8], syntax::Expr>,
  alt!(
    map!(context, syntax::Expr::Context) |
    map!(identifier, syntax::Expr::Variable)
  )
);

named!(pub level<&[u8], syntax::Expr>,
  ws!(do_parse!(
    sensitivity: identifier >>
    char!(':') >>
    categories: category_range >>

    (syntax::Expr::Level(syntax::LevelExpr {
      sensitivity,
      categories: Box::from(categories)
    }))
  ))
);

named!(pub level_range<&[u8], syntax::Expr>,
  ws!(do_parse!(
    low: level >>
    char!('-') >>
    high: level >>

    (syntax::Expr::LevelRange(
      Box::from(low), Box::from(high)
    ))
  ))
);

named!(pub category_range<&[u8], syntax::Expr>,
  ws!(do_parse!(
    low: identifier >>
    char!('.') >>
    high: identifier >>

    (syntax::Expr::CategoryRange(low, high))
  ))
);

named!(pub context<&[u8], syntax::ContextExpr>,
  ws!(do_parse!(
      user_id: identifier >>
      char!(':') >>
      role_id: identifier >>
      char!(':') >>
      type_id: identifier >>
      level_range: opt!(complete!(preceded!(char!(':'), level_range))) >>

      (syntax::ContextExpr {
        user_id,
        role_id,
        type_id,
        level_range: level_range.map(|v| Box::from(v))
      })
  ))
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
named!(pub block_declaration<&[u8], syntax::Block>,
  ws!(do_parse!(
    is_abstract: opt!(tag!("abstract")) >>
    qualifier: type_specifier >>
    name: identifier >> 
    char!('{') >>
    statements: many0!(statement) >>
    char!('}') >>

    (syntax::Block {
      is_abstract: is_abstract.is_some(),
      qualifier,
      name,
      statements
    })
  ))
);

#[cfg(test)]
mod tests {

    use super::*;
    use syntax::*;

    fn parse<O, P>(input: &str, parser: P) -> O
    where
        P: Fn(&[u8]) -> IResult<&[u8], O>,
    {
        let bytes = input.as_bytes();
        let (remaining, result) = parser(bytes).unwrap();

        result
    }

    #[test]
    pub fn parse_type_specifier() {
        assert_eq!(
            parse::<SymbolType, _>("type", type_specifier),
            SymbolType::Type
        );
    }

    #[test]
    pub fn parse_context_expr() {
        let result = parse::<ContextExpr, _>("user:role:type", context);

        assert_eq!("user", result.user_id);
        assert_eq!("role", result.role_id);
        assert_eq!("type", result.type_id);
        assert_eq!(true, result.level_range.is_none());
    }

    #[test]
    pub fn parse_block_decl() {
        let result = parse::<Block, _>("block abc {}", block_declaration);

        assert_eq!("abc", result.name);
        assert_eq!(false, result.is_abstract);
        assert_eq!(BlockType::Block, result.qualifier);
    }

    #[test]
    pub fn parse_abstract_block_decl() {
        let result = parse::<Block, _>("abstract block abc {}", block_declaration);

        assert_eq!("abc", result.name);
        assert_eq!(true, result.is_abstract);
        assert_eq!(BlockType::Block, result.qualifier);
    }

    #[test]
    pub fn parse_symbol_decl() {
        let result = parse::<Declaration, _>("type_attribute my_type;", symbol_declaration);

        assert_eq!(
            Declaration::Symbol(SymbolType::TypeAttribute, "my_type".into()),
            result
        );
    }
}