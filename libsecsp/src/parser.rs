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
    map!(declaration, Statement::Declaration) | macro_call | if_else
  )
);

// Parse a list of 0 or more statements.
named!(pub statement_list<&[u8], Vec<Statement>>, many0!(statement));

/// Parse either a block or symbol declaration.
named!(pub declaration<&[u8], Declaration>,
  alt!(
    block_declaration
    | macro_declaration
    | symbol_declaration 
  )
);

/// Parse a single named `Symbol` declaration.
named!(pub symbol_declaration<&[u8], Declaration>,
  ws!(do_parse!(
    qualifier: type_specifier >>
    name: identifier >>
    initializer: opt!(preceded!(tag!("="), expr)) >>
    char!(';') >>

    (Declaration::Symbol {qualifier, name, initializer})
  ))
);

/// Parse a `block` or `optional` container, named by an `Identifer` and containing
/// a list of 0 or more `Statement`s.
named!(pub block_declaration<&[u8], Declaration>,
  ws!(do_parse!(
    is_abstract: opt!(tag!("abstract")) >>
    qualifier: type_specifier >>
    name: identifier >> 
    char!('{') >>
    statements: many0!(statement) >>
    char!('}') >>

    (Declaration::Block {
      is_abstract: is_abstract.is_some(),
      qualifier,
      name,
      statements
    })
  ))
);

named!(pub macro_declaration<&[u8], Declaration>,
    ws!(do_parse!(
        tag!("macro") >>
        name: identifier >>
        parameters: delimited!(tag!("("), macro_param_list, tag!(")")) >>
        tag!("{") >>
        statements: statement_list >>
        tag!("}") >>

        (Declaration::Macro {
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

named!(pub if_else<Statement>,
    dbg_dmp!(ws!(do_parse!(
        tag!("if") >>
        condition: expr >>
        then_block: delimited!(char!('{'), statement_list, char!('}')) >>
        else_ifs: many0!(else_if) >>
        else_block: opt!(complete!(do_parse!(tag!("else") >> block: delimited!(char!('{'), statement_list, char!('}')) >> (block)))) >>

        (Statement::IfElse {
            condition,
            then_block,
            else_ifs,
            else_block,
        })
    )))
);


named!(pub else_if<(Expr, Vec<Statement>)>,
    ws!(do_parse!(
        tag!("elseif") >>
        condition: expr >>
        tag!("{") >>
        then_block: statement_list >>
        tag!("}") >>

        (condition, then_block)
    ))
);

named!(pub expr<&[u8], Expr>,
  alt_complete!(
    context
    | level_range 
    | category_range
    | variable
  )
);

named!(pub level<&[u8], Expr>,
    do_parse!(
        sensitivity: identifier >>
        tag!(":") >>
        categories: category_range_or_id >>

        (Expr::Level {
            sensitivity,
            categories: Box::from(categories)
        })
    ) 
);

named!(level_or_id<&[u8], Expr>, alt_complete!(level | variable));

named!(pub level_range<&[u8], Expr>,
    ws!(do_parse!(
        range: separated_pair!(level_or_id, eat_separator!(&b"-"[..]), level_or_id) >>

        (Expr::LevelRange(
            Box::from(range.0), Box::from(range.1)
        ))
    )) 
);

named!(category_range_or_id<&[u8], Expr>, alt_complete!(category_range | variable));

named!(pub category_range<&[u8], Expr>,
  ws!(do_parse!(
        range: separated_pair!(identifier, eat_separator!(&b"."[..]), identifier) >>

        (Expr::CategoryRange(
            range.0, range.1
        ))
    )) 
);

named!(pub context<&[u8], Expr>,
  ws!(do_parse!(
      user_id: identifier >>
      char!(':') >>
      role_id: identifier >>
      char!(':') >>
      type_id: identifier >>
      level_range: opt!(complete!(preceded!(char!(':'), level_range))) >>
      
      (Expr::Context {
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
        let result = parse::<Expr, _>("user:role:type", context);

        match result {
            Expr::Context {
                user_id,
                role_id,
                type_id,
                level_range,
            } => {
                assert_eq!("user", user_id);
                assert_eq!("role", role_id);
                assert_eq!("type", type_id);
                assert_eq!(true, level_range.is_none());
            }
            _ => panic!("Invalid value parsed"),
        }
    }

    #[test]
    pub fn parse_levelrange_expr() {
        let result = parse::<Expr, _>("s0-s1", level_range);

        if let Expr::LevelRange(low, high) = result {
            assert_eq!(Expr::var("s0"), *low);
            assert_eq!(Expr::var("s1"), *high);
        } else {
            panic!("Invalid value parsed");
        }
    }

    #[test]
    pub fn parse_context_expr_with_levelrange() {
        let result = parse::<Expr, _>("user:role:type:s0 - s1", context);

        match result {
            Expr::Context {
                user_id,
                role_id,
                type_id,
                level_range,
            } => {
                if let &Expr::LevelRange(ref low, ref high) = level_range.unwrap().as_ref() {
                    assert_eq!(Expr::var("s0"), **low);
                    assert_eq!(Expr::var("s1"), **high);
                } else {
                    panic!("No level range found");
                }
            }
            _ => panic!("Invalid value parsed"),
        }
    }


    #[test]
    pub fn parse_block_decl() {
        let result = parse::<Declaration, _>("abstract block abc {}", block_declaration);

        match result {
            Declaration::Block {
                is_abstract,
                qualifier,
                statements,
                name,
            } => {
                assert_eq!(true, is_abstract);
                assert_eq!("abc", name);
                assert_eq!(BlockType::Block, qualifier);
            }
            _ => panic!("Invalid value"),
        }
    }

    #[test]
    pub fn parse_abstract_block_decl() {
        let result = parse::<Declaration, _>("abstract block abc {}", block_declaration);

        match result {
            Declaration::Block {
                is_abstract,
                qualifier,
                statements,
                name,
            } => assert_eq!(true, is_abstract),
            _ => panic!("Invalid value"),
        }
    }

    #[test]
    pub fn parse_symbol_decl() {
        let result = parse::<Declaration, _>("type_attribute my_type;", symbol_declaration);

        match result {
            Declaration::Symbol {
                qualifier,
                name,
                initializer,
            } => {
                assert_eq!(SymbolType::TypeAttribute, qualifier);
                assert_eq!("my_type", name);
                assert_eq!(None, initializer)
            }
            _ => panic!("Invalid value parsed"),
        }
    }

    #[test]
    pub fn parse_symbol_decl_with_initializer() {
        let result = parse::<Declaration, _>(
            "context my_context = user:role:type:s0-s1;",
            symbol_declaration,
        );

        match result {
            Declaration::Symbol {
                qualifier,
                name,
                initializer,
            } => {
                assert_eq!(SymbolType::Context, qualifier);
                assert_eq!("my_context", name);

                match initializer {
                    Some(Expr::Context {
                             user_id,
                             role_id,
                             type_id,
                             level_range,
                         }) => {}
                    _ => panic!("No initializer found"),
                }
            }
            _ => panic!("Invalid value parsed"),
        }
    }

    #[test]
    pub fn parse_macro_decl() {
        let result = parse::<Declaration, _>(
            "macro my_macro(type v, type v1) {

            }",
            macro_declaration,
        );

        match result {
            Declaration::Macro {
                name,
                parameters,
                statements,
            } => {
                assert_eq!("my_macro", name);
                assert_eq!("v", parameters[0].name);
                assert_eq!("v1", parameters[1].name);
            }
            _ => panic!("Invalid value parsed"),
        }
    }

    #[test]
    pub fn parse_macro_call() {
        let result = parse::<Statement, _>("my_macro(type_name);", macro_call);

        if let Statement::MacroCall(ref name, ref params) = result {
            assert_eq!("my_macro", name);
            assert_eq!(Expr::var("type_name"), params[0])
        } else {
            panic!("Invalid value parsed");
        }
    }

    #[test]
    pub fn parse_if_then_else() {
        let result = parse::<Statement, _>("if my_bool {} else{}", if_else);

        match result {
            Statement::IfElse {
                condition,
                then_block,
                else_ifs,
                else_block,
            } => {
                assert_eq!(Expr::var("my_bool"), condition);
                assert_eq!(Some(vec![]), else_block);
            }
            _ => panic!("Invalid value parsed"),
        }
    }

    #[test]
    pub fn parse_if() {
        let result = parse::<Statement, _>("if my_bool {}", if_else);

        match result {
            Statement::IfElse {
                condition,
                then_block,
                else_ifs,
                else_block,
            } => {
                assert_eq!(Expr::var("my_bool"), condition);
                assert_eq!(None, else_block);
            }
            _ => panic!("Invalid value parsed"),
        }
    }
}