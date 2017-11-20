use ast::*;
use expr::*;
use name::*;
use security_attributes::*;

named!(pub label<&[u8], Label>,
    alt!(
        file_context_block | file_context_single
    )
);

named!(pub file_context_block<&[u8], Label>,
    ws!(do_parse!(
        tag!("file_contexts") >>
        labels: delimited!(char!('{'), many0!(file_context_spec), char!('}')) >>

        (Label::FileContextBlock(labels))
    ))
);

named!(pub file_context_single<&[u8], Label>,
    ws!(do_parse!(
        tag!("file_context") >>
        spec: file_context_spec >>
        char!(';') >>

        (spec)
    ))
);

named!(file_context_spec<&[u8], Label>,
    ws!(do_parse!(
        path: string_literal >>
        file_type: opt!(complete!(type_specifier)) >>
        context: expr >>

        (Label::FileContext {
            path,
            file_type,
            context: Box::from(context)
        })
    ))
);

#[cfg(test)]
mod tests {
    use super::*;
    use testing::parse;

    #[test]
    pub fn parse_file_context_single_without_type() {
        let actual = parse::<Label, _>("file_context \"/usr/lib64\" context_id;", label);
        let expected = Label::FileContext {
            path: "/usr/lib64".to_string(),
            file_type: None,
            context: Box::from(Expr::var("context_id"))
        };

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn parse_file_context_single_with_type() {
        let actual = parse::<Label, _>("file_context \"/usr/lib64\" dir context_id;", label);
        let expected = Label::FileContext {
            path: "/usr/lib64".to_string(),
            file_type: Some(FileType::Dir),
            context: Box::from(Expr::var("context_id"))
        };

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn parse_file_context_block() {
        let actual = parse::<Label, _>("file_contexts {
            \"/usr/lib64\" context_id
            \"/usr/bin\" dir other_context_id
        }", label);

        let expected = Label::FileContextBlock(vec![
            Label::FileContext {
                path: "/usr/lib64".to_string(),
                file_type: None,
                context: Box::from(Expr::var("context_id"))
            },
            Label::FileContext {
                path: "/usr/bin".to_string(),
                file_type: Some(FileType::Dir),
                context: Box::from(Expr::var("other_context_id"))
            }
        ]);

        assert_eq!(expected, actual);
    }
}