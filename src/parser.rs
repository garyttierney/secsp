use nom::{Err as NomErr, ErrorKind, IResult};
use nom::IResult::*;
use syntax::*;

/// Check if the given byte is valid in an identifier.
fn is_identifier(c: u8) -> bool {
    let ch = char::from(c);
    ch.is_alphanumeric() || ch == '_'
}

named!(identifier_raw <&[u8], &[u8]>, take_while1!(is_identifier));
named!(identifier <&[u8], Identifier>, map!(identifier_raw, |bytes: &[u8]| String::from_utf8(bytes.to_vec()).unwrap()));

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

/// Parse a declaration as a statement.
named!(pub statement<&[u8], Statement>,
  alt!(
    map!(declaration, Statement::Declaration) | macro_call
  )
);

/// Parse either a block or symbol declaration.
named!(pub declaration<&[u8], Declaration>,
  alt!(
    map!(block_declaration, Declaration::Block) | symbol_declaration
  )
);

/// Parse a single named `Symbol` declaration.
named!(pub symbol_declaration<&[u8], Declaration>,
  ws!(do_parse!(
    qualifier: type_specifier >>
    name: identifier >>
    initializer: opt!(preceded!(tag!("="), expr)) >>
    char!(';') >>

    (Declaration::Symbol(qualifier, name, initializer))
  ))
);

/// Parse a `block` or `optional` container, named by an `Identifer` and containing
/// a list of 0 or more `Statement`s.
named!(pub block_declaration<&[u8], Block>,
  ws!(do_parse!(
    is_abstract: opt!(tag!("abstract")) >>
    qualifier: type_specifier >>
    name: identifier >> 
    char!('{') >>
    statements: many0!(statement) >>
    char!('}') >>

    (Block {
      is_abstract: is_abstract.is_some(),
      qualifier,
      name,
      statements
    })
  ))
);

named!(pub macro_declaration<&[u8], Macro>,
    ws!(do_parse!(
        tag!("macro") >>
        name: identifier >>
        parameters: delimited!(tag!("("), macro_param_list, tag!(")")) >>
        tag!("{") >>
        statements: many0!(statement) >>
        tag!("}") >>

        (Macro {
            name,
            parameters,
            statements
        })
    ))
);

named!(pub macro_param_list<&[u8], Vec<MacroParameter>>,
    ws!(do_parse!(
        first_param: macro_param >>
        rest_params: many0!(ws!(do_parse!(char!(',') >> param: macro_param >> (param)))) >>

        ({
            let mut params = rest_params.clone();
            params.insert(0, first_param);

            params
        })
    ))
);

named!(pub macro_param<&[u8], MacroParameter>,
    ws!(do_parse!(
        qualifier: type_specifier >>
        name: identifier >>

        (MacroParameter {
            qualifier, name
        })
    ))
);

named!(pub macro_call<&[u8], Statement>,
    ws!(do_parse!(
        name: identifier >>
        arguments: delimited!(tag!("("), macro_argument_list, tag!(")")) >>
        tag!(";") >>

        (Statement::MacroCall(name, arguments))
    ))
);

named!(pub macro_argument_list<&[u8], Vec<Expr>>,
    ws!(do_parse!(
        first_param: expr >>
        rest_params: many0!(ws!(do_parse!(char!(',') >> param: expr >> (param)))) >>

        ({
            let mut params = rest_params.clone();
            params.insert(0, first_param);

            params
        })
    ))
);

named!(pub expr<&[u8], Expr>,
  alt!(
    map!(context, Expr::Context)
    | level_range
    | category_range
    | variable
  )
);

named!(pub level<&[u8], Expr>,
  alt_complete!(
    do_parse!(
        sensitivity: identifier >>
        categories: category_range >>

        (Expr::Level(LevelExpr {
            sensitivity,
            categories: Box::from(categories)
        }))
    )
    | variable
  )
);

named!(pub level_range<&[u8], Expr>,
    ws!(do_parse!(
        range: separated_pair!(level, eat_separator!(&b"-"[..]), level) >>

        (Expr::LevelRange(
            Box::from(range.0), Box::from(range.1)
        ))
    ))
);

named!(pub category_range<&[u8], Expr>,
  ws!(do_parse!(
        range: separated_pair!(identifier, eat_separator!(&b"."[..]), identifier) >>

        (Expr::CategoryRange(
            range.0, range.1
        ))
    ))
);

named!(pub context<&[u8], ContextExpr>,
  ws!(do_parse!(
      user_id: identifier >>
      char!(':') >>
      role_id: identifier >>
      char!(':') >>
      type_id: identifier >>
      level_range: opt!(complete!(preceded!(char!(':'), level_range))) >>

      (ContextExpr {
        user_id,
        role_id,
        type_id,
        level_range: level_range.map(|v| Box::from(v))
      })
  ))
);

#[cfg(test)]
mod tests {

    use super::*;

    fn parse<O, P>(input: &str, parser: P) -> O
    where
        P: Fn(&[u8]) -> IResult<&[u8], O>,
    {
        let bytes = input.as_bytes();
        let result = parser(bytes);

        if result.is_err() {
            panic!("Parse error: {}", result.unwrap_err());
        }

        let (remaining, output) = result.unwrap();

        output
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
    pub fn parse_levelrange_expr() {
        let result = parse::<Expr, _>("s0-s1", level_range);

        if let Expr::LevelRange(low, high) = result {
            assert_eq!(Expr::Variable("s0".into()), *low);
            assert_eq!(Expr::Variable("s1".into()), *high);
        } else {
            panic!("Invalid value parsed");
        }
    }

    #[test]
    pub fn parse_context_expr_with_levelrange() {
        let result = parse::<ContextExpr, _>("user:role:type:s0 - s1", context);
        let level_range = result.level_range.unwrap();

        if let &Expr::LevelRange(ref low, ref high) = level_range.as_ref() {
            assert_eq!(Expr::Variable("s0".into()), **low);
            assert_eq!(Expr::Variable("s1".into()), **high);
        } else {
            panic!("Invalid value parsed");
        }
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
            Declaration::Symbol(SymbolType::TypeAttribute, "my_type".into(), None),
            result
        );
    }

    #[test]
    pub fn parse_symbol_decl_with_initializer() {
        let result = parse::<Declaration, _>(
            "context my_context = user:role:type:s0-s1;",
            symbol_declaration,
        );

        if let Declaration::Symbol(ref sym_type, ref name, ref initializer) = result {
            assert_eq!(SymbolType::Context, *sym_type);
            assert_eq!("my_context", *name);

            match *initializer {
                Some(Expr::Context(_)) => {}
                _ => panic!("No initializer found"),
            }
        } else {
            panic!("Invalid value parsed");
        }
    }

    #[test]
    pub fn parse_macro_decl() {
        let result = parse::<Macro, _>(
            "macro my_macro(type v, type v1) {

            }",
            macro_declaration,
        );

        assert_eq!("my_macro", result.name);

        let params = result.parameters;

        assert_eq!("v", params[0].name);
        assert_eq!("v1", params[1].name);
    }

    #[test]
    pub fn parse_macro_call() {
        let result = parse::<Statement, _>("my_macro(type_name);", macro_call);

        if let Statement::MacroCall(ref name, ref params) = result {
            assert_eq!("my_macro", name);
            assert_eq!(Expr::Variable("type_name".into()), params[0])
        } else {
            panic!("Invalid value parsed");
        }
    }
}