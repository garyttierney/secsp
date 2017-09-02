use secsp::syntax::*;
use std::io::{Error as IoError, Write};

pub fn show_statement<F>(f: &mut F, stmt: &Statement) -> Result<(), IoError>
where
    F: Write,
{
    match *stmt {
        Statement::Declaration(ref decl) => show_declaration(f, decl)?,
        Statement::MacroCall(ref id, ref parameters) => show_macro_call(f, id, parameters)?, 
    };

    try!(write!(f, "\n"));

    Ok(())
}

pub fn show_macro_call<F>(f: &mut F, id: &String, arguments: &Vec<Expr>) -> Result<(), IoError>
where
    F: Write,
{
    try!(write!(f, "(call {} (", id));

    for argument in arguments {
        try!(show_expr(f, argument));
    }

    try!(write!(f, "))"));

    Ok(())
}

pub fn show_declaration<F>(f: &mut F, decl: &Declaration) -> Result<(), IoError>
where
    F: Write,
{
    match *decl {
        Declaration::Symbol {
            ref qualifier,
            ref name,
            ref initializer,
        } => show_symbol_declaration(f, qualifier, name, initializer)?,
        Declaration::Block {
            ref is_abstract,
            ref qualifier,
            ref name,
            ref statements,
        } => show_block_declaration(f, *is_abstract, qualifier, name, statements)?,
        Declaration::Macro {
            ref name,
            ref parameters,
            ref statements,
        } => show_macro_declaration(f, name, parameters, statements)?,
    };

    Ok(())
}

pub fn show_macro_declaration<F>(
    f: &mut F,
    name: &String,
    parameters: &Vec<MacroParameter>,
    statements: &Vec<Statement>,
) -> Result<(), IoError>
where
    F: Write,
{
    try!(write!(f, "(macro {}(", name));

    for parameter in parameters {
        try!(write!(
            f,
            "({} {})",
            parameter.qualifier.to(),
            parameter.name
        ));
    }

    try!(writeln!(f, ")"));

    for statement in statements {
        show_statement(f, statement)?;
    }

    try!(writeln!(f, ")"));

    Ok(())
}

pub fn show_block_declaration<F>(
    f: &mut F,
    is_abstract: bool,
    qualifier: &BlockType,
    name: &String,
    statements: &Vec<Statement>,
) -> Result<(), IoError>
where
    F: Write,
{
    try!(writeln!(f, "({} {}", qualifier.to(), name));

    if is_abstract {
        try!(writeln!(f, "(blockabstract {})", name));
    }

    for statement in statements {
        try!(show_statement(f, statement));
    }

    try!(writeln!(f, ")"));

    Ok(())
}

pub fn show_symbol_declaration<F>(
    f: &mut F,
    qualifier: &SymbolType,
    name: &String,
    initializer: &Option<Expr>,
) -> Result<(), IoError>
where
    F: Write,
{
    try!(write!(f, "({} {})", qualifier.to(), name));

    Ok(())
}

pub fn show_expr<F>(f: &mut F, expr: &Expr) -> Result<(), IoError>
where
    F: Write,
{
    match *expr {
        Expr::Variable(ref id) => try!(write!(f, "{}", id)),
        _ => {}
    };

    Ok(())
}