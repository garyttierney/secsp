use ast::*;
use name::*;
use security_attributes::*;

named!(file_context, <&[u8], Label>,
    ws!(
        do_parse!(
            path: << string_literal,
            file_type: << type_specifier,
            context: expr <<

            (Label::FileContext {
                path,
                file_type,
                context: Box::from(context)
            })
        )
    ))
