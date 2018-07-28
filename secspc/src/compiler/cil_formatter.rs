
extern crate symbolic_expressions;

use self::symbolic_expressions::Sexp;
use self::symbolic_expressions::Error;
use self::symbolic_expressions::Formatter;
use std::io::Write;

pub struct CilSexprState {
    current_child_index: u32,
}

impl Default for CilSexprState {
    fn default() -> Self {
        CilSexprState { current_child_index: 0 }
    }
}

pub struct CilFormatter {
    indent_width: u32,
    column_width: u32,
    current_indent: u32,
    current_column: u64,
    current_line: u64,
    mode_stack: Vec<FormatMode>,
    state_stack: Vec<CilSexprState>,
}

impl Default for CilFormatter {
    fn default() -> Self {
        CilFormatter {
            indent_width: 4,
            column_width: 100,
            current_indent: 0,
            current_column: 0,
            current_line: 0,
            mode_stack: vec![],
            state_stack: vec![],
        }
    }
}

enum FormatMode {
    // Make a line break after the given number of elements.
    LineBreakAfter(u32),
    LineBreakOnClose,
    None,
}


impl Formatter for CilFormatter {
    fn open<W>(&mut self, writer: &mut W, value: Option<&Sexp>) -> Result<(), Error>
    where
        W: Write,
    {
        write!(writer, "(\n").map_err(From::from)
    }
    fn element<W>(&mut self, writer: &mut W, _value: &Sexp) -> Result<(), Error>
    where
        W: Write,
    {
        write!(writer, " ").map_err(From::from)
    }

    fn close<W>(&mut self, writer: &mut W) -> Result<(), Error>
    where
        W: Write,
    {
        write!(writer, ")\n").map_err(From::from)
    }
}
