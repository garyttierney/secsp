use regex::{Regex, RegexBuilder};
use text_unit::TextRange;

use crate::ast::SyntaxNode;
use crate::ast::SyntaxNodeRef;
use crate::parser::parse_file;
use text_unit::TextUnit;

#[derive(Debug)]
struct Assertion {
    ty: String,
    start_pos: usize,
    end_pos: usize,
}

pub(crate) fn test_parser(text: &str) {
    let regex: Regex = RegexBuilder::new(r#"<marker type="([a-zA-Z\(\)]+)">(.*)</marker>"#)
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let ws_regex = Regex::new(r#"\s"#).unwrap();

    let mut code = text.to_owned();
    let mut assertions: Vec<Assertion> = vec![];

    for capture in regex.captures_iter(text) {
        let full_match = &capture[0];
        let open_tag = &capture[1];
        let content = &capture[2];

        code = code.replace(full_match, content);

        let code_start_pos = code.find(content).unwrap();
        let assertion = Assertion {
            ty: open_tag.to_owned(),
            start_pos: code_start_pos,
            end_pos: code_start_pos + content.len() - 1,
        };

        assertions.push(assertion);
    }

    let ast = parse_file(code.as_str());

    if assertions.is_empty() {
        panic!("No assertions found");
    }

    for assertion in assertions.into_iter() {
        let node = ast.syntax.borrowed().covering_node(TextRange::from_to(
            TextUnit::from_usize(assertion.start_pos),
            TextUnit::from_usize(assertion.end_pos),
        ));

        let raw_kind = format!("{:#?}", node.kind());
        let kind = ws_regex.replace_all(raw_kind.as_str(), "");
        let expected_kind = assertion.ty;

        assert_eq!(expected_kind.to_lowercase(), kind.to_lowercase());
    }
}
