extern crate symbolic_expressions;

use self::cil_types::ToCil;
use self::symbolic_expressions::Rules;
use self::symbolic_expressions::ser::to_writer_with_rules;

use secsp::ast::*;
use std::io::Error;
use std::io::Write;

mod cil_types;
pub fn print<F>(out: &mut F, statements: &Vec<Statement>) -> Result<(), Error>
where
    F: Write,
{
    for statement in statements {
        let mut rules = Rules::new();

        to_writer_with_rules(out, rules, &statement.into_sexp());
    }

    Ok(())
}