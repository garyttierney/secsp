use std::fmt::Write;

use regex::{Regex, RegexBuilder};
use rowan::WalkEvent;
use text_unit::TextRange;
use text_unit::TextUnit;

use secsp_parser::syntax::SyntaxNode;

use crate::ast::AstNode;
use crate::SourceFile;

mod def;
mod expr;
mod stmt;

#[derive(Debug)]
struct Assertion {
    ty: String,
    range: TextRange,
}

pub fn ast_to_string(source: &SyntaxNode) -> String {
    let mut indent = 0;
    let mut out = String::new();
    let regex = Regex::new(r#"[\s\n ]+"#).unwrap();

    for event in source.preorder() {
        match event {
            WalkEvent::Enter(el) => {
                let code_inline = format!("{:#}", el);
                let code_inline_normalized = regex.replace_all(&code_inline, " ");

                writeln!(
                    out,
                    "{:indent$}{:?} {}",
                    "",
                    el,
                    code_inline_normalized,
                    indent = indent
                )
                .unwrap();

                indent += 2;
            }
            WalkEvent::Leave(_) => indent -= 2,
        };
    }

    out
}

pub(crate) fn test_parser(text: &str) {
    let (code, assertions) = strip_markers(0.into(), text);

    if assertions.is_empty() {
        panic!("No assertions found");
    }

    let ast = SourceFile::parse(code.as_str()).tree();
    let ws_regex = Regex::new(r#"\s"#).unwrap();

    for assertion in assertions.into_iter() {
        let node = ast.syntax().covering_element(assertion.range);
        let node_kind = node.kind();
        let raw_kind = format!("{:#?}", node_kind);
        let kind = ws_regex.replace_all(raw_kind.as_str(), "");
        let expected_kind = assertion.ty;

        assert_eq!(
            expected_kind.to_lowercase(),
            kind.to_lowercase(),
            "Expected {} at {:?}. Resulting parse tree:\n {}",
            expected_kind,
            assertion.range,
            ast_to_string(ast.syntax())
        );
    }
}

fn strip_markers(offset: TextUnit, text: &str) -> (String, Vec<Assertion>) {
    let regex: Regex = RegexBuilder::new(r#"(<marker type="([a-zA-Z_\(\)]+)">)(.*)(</marker>)"#)
        .multi_line(true)
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let mut code = text.to_owned();
    let mut assertions: Vec<Assertion> = vec![];
    let mut capture_locations = regex.capture_locations();

    while let Some(m) = regex.captures_read(&mut capture_locations, code.as_str()) {
        let (start, end) = (m.start(), m.end());
        let (marker_start, marker_end) = capture_locations.get(1).unwrap();
        let (type_start, type_end) = capture_locations.get(2).unwrap();
        let (contents_start, contents_end) = capture_locations.get(3).unwrap();

        let contents_offset =
            offset + TextUnit::from_usize(contents_start - (marker_end - marker_start));

        let ty = &code[type_start..type_end];
        let contents = &code[contents_start..contents_end];

        let (stripped_contents, mut submatches) = strip_markers(contents_offset, contents);
        let range = TextRange::offset_len(contents_offset, TextUnit::of_str(&stripped_contents));

        assertions.push(Assertion {
            ty: ty.to_string(),
            range,
        });
        assertions.append(&mut submatches);
        code.replace_range(start..end, stripped_contents.as_str());
    }

    (code, assertions)
}
