use ast::*;
use name::*;
use expr::*;

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
    ws!(do_parse!(
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
    ))
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

#[cfg(test)]
mod tests {

    use super::*;
    use testing::parse;

    #[test]
    pub fn parse_block_decl() {
        let result = parse::<Declaration, _>("abstract block abc {}", block_declaration);

        match result {
            Declaration::Block {
                is_abstract,
                qualifier,
                name,
                ..
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
            Declaration::Block { is_abstract, .. } => assert_eq!(true, is_abstract),
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
            Declaration::Macro { name, parameters, .. } => {
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
                else_block,
                ..
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
                else_block,
                ..
            } => {
                assert_eq!(Expr::var("my_bool"), condition);
                assert_eq!(None, else_block);
            }
            _ => panic!("Invalid value parsed"),
        }
    }
}