use secsp::ast::*;
use std::io::{Error as IoError, Write};

pub trait CilType: TypeSpecifier {
    fn requires_initializer(&self) -> bool;
}

impl CilType for SymbolType {
    fn requires_initializer(&self) -> bool {
        match *self {
            SymbolType::Context => true,
            _ => false,
        }
    }
}

pub fn show_statement<F>(f: &mut F, stmt: &Statement) -> Result<(), IoError>
where
    F: Write,
{
    match *stmt {
        Statement::Declaration(ref decl) => show_declaration(f, decl)?,
        Statement::MacroCall(ref id, ref parameters) => show_macro_call(f, id, parameters)?, 
        Statement::IfElse {
            ref condition,
            ref then_block,
            ref else_ifs,
            ref else_block,
        } => show_if_statement(f, condition, then_block, else_ifs, else_block)?,
        _ => {}
    };

    try!(write!(f, "\n"));

    Ok(())
}

pub fn show_if_statement<F>(
    f: &mut F,
    condition: &Expr,
    block: &Vec<Statement>,
    else_ifs: &Vec<(Expr, Vec<Statement>)>,
    else_block: &Option<Vec<Statement>>,
) -> Result<(), IoError>
where
    F: Write,
{
    try!(write!(f, "(booleanif "));
    show_expr(f, condition)?;
    try!(write!(f, "\n(true \n"));

    for statement in block {
        show_statement(f, statement)?;
    }

    try!(write!(f, ")\n"));

    if let Some(ref statements) = *else_block {
        try!(write!(f, "(false \n"));

        for statement in statements {
            show_statement(f, statement)?;
        }

        try!(write!(f, ")\n"));
    }

    try!(write!(f, ")\n)"));

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
        } => show_symbol_declaration(f, qualifier, name, initializer.as_ref())?,
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
            parameter.qualifier.to_cil(),
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
    try!(writeln!(f, "({} {}", qualifier.to_cil(), name));

    if is_abstract {
        try!(writeln!(f, "(blockabstract {})", name));
    }

    for statement in statements {
        try!(show_statement(f, statement));
    }

    try!(writeln!(f, ")"));

    Ok(())
}

pub fn show_symbol_declaration<F, T: CilType>(
    f: &mut F,
    qualifier: &T,
    name: &String,
    _initializer: Option<&Expr>,
) -> Result<(), IoError>
where
    F: Write,
{
    if qualifier.requires_initializer() {
        let expr = _initializer.unwrap_or_else(|| panic!("No initializer given"));
        try!(write!(f, "({} {} ", qualifier.to_cil(), name));
        show_expr(f, expr)?;
        try!(write!(f, ")"));
    } else {
        try!(write!(f, "({} {})", qualifier.to_cil(), name));
    }

    Ok(())
}

pub fn show_expr<F>(f: &mut F, expr: &Expr) -> Result<(), IoError>
where
    F: Write,
{
    match *expr {
        Expr::Variable(ref id) => try!(write!(f, "{}", id)),
        Expr::Context {
            ref user_id,
            ref role_id,
            ref type_id,
            ref level_range,
        } => show_context_expr(f, user_id, role_id, type_id, level_range.as_ref())?,
        Expr::LevelRange(ref low, ref high) => {
            show_level_range_expr(f, low.as_ref(), high.as_ref())?
        }
        Expr::Level {
            ref sensitivity,
            ref categories,
        } => show_level_expr(f, sensitivity, categories.as_ref())?,
        Expr::Binary(ref lhs, ref op, ref rhs) => {
            show_binary_expr(f, lhs.as_ref(), op, rhs.as_ref())?
        }
        Expr::Unary(ref op, ref expr) => show_unary_expr(f, op, expr.as_ref())?,
        _ => {}
    };

    Ok(())
}

pub fn show_unary_expr<F>(f: &mut F, op: &UnaryOp, expr: &Expr) -> Result<(), IoError>
where
    F: Write,
{
    try!(write!(f, "("));
    let op_value = match *op {
        UnaryOp::ConditionalNot => "not",
        _ => "Unknown",
    };

    try!(write!(f, "{} ", op_value));
    show_expr(f, expr)?;
    try!(write!(f, ")"));

    Ok(())
}

pub fn show_binary_expr<F>(f: &mut F, lhs: &Expr, op: &BinaryOp, rhs: &Expr) -> Result<(), IoError>
where
    F: Write,
{
    try!(write!(f, "("));
    let op_value = match *op {
        BinaryOp::ConditionalAnd => "and",
        BinaryOp::ConditionalOr => "or",
        BinaryOp::ConditionalXor => "xor",
        _ => "Unknown",
    };

    try!(write!(f, "{} ", op_value));
    show_expr(f, lhs)?;
    try!(write!(f, " "));
    show_expr(f, rhs)?;
    try!(write!(f, ")"));

    Ok(())
}

pub fn show_level_range_expr<F>(f: &mut F, low: &Expr, high: &Expr) -> Result<(), IoError>
where
    F: Write,
{

    try!(write!(f, "("));
    show_expr(f, low)?;
    try!(write!(f, " "));
    show_expr(f, high)?;
    try!(write!(f, ")"));

    Ok(())
}

pub fn show_level_expr<F>(f: &mut F, sensitivity: &String, categories: &Expr) -> Result<(), IoError>
where
    F: Write,
{
    try!(write!(f, "({} (", sensitivity));
    show_expr(f, categories)?;
    try!(write!(f, "))"));

    Ok(())
}

pub fn show_context_expr<F>(
    f: &mut F,
    user_id: &String,
    role_id: &String,
    type_id: &String,
    level_range_ref: Option<&Box<Expr>>,
) -> Result<(), IoError>
where
    F: Write,
{
    if level_range_ref.is_some() {
        let level_range_ptr: &Box<Expr> = level_range_ref.unwrap();
        let level_range = level_range_ptr.as_ref();

        try!(write!(f, "({} {} {} ", user_id, role_id, type_id));
        show_expr(f, level_range)?;
        try!(write!(f, ")"));
    } else {
        try!(write!(f, "({} {} {})", user_id, role_id, type_id));
    }

    Ok(())
}
