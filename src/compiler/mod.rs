mod cil_types;

use self::cil_types::ToCil;
use secsp::ast::*;
use std::io::Error;
use std::io::Write;

pub fn print<F>(out: &mut F, statements: &Vec<Statement>) -> Result<(), Error>
where
    F: Write,
{
    for statement in statements {
        write!(out, "{}", statement.into_sexp())?;
    }
    
    Ok(())
}