extern crate symbolic_expressions;

use self::cil_types::ToCil;
use self::symbolic_expressions::ser::to_writer_with_formatter;

use secsp::ast::*;
use std::io::Error;
use std::io::Write;

mod cil_formatter;
mod cil_types;

pub fn emit<F>(out: &mut F, statements: &[Statement]) -> Result<(), Error>
where
    F: Write,
{
    for statement in statements {
        let formatter = cil_formatter::CilFormatter::default();
        to_writer_with_formatter(out, formatter, &statement.into_sexp());
    }

    Ok(())
}
