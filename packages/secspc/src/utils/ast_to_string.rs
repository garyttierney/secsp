use std::fmt::Write;

use secsp_syntax::ast::types::WalkEvent;
use secsp_syntax::ast::AstNode;
use secsp_syntax::ast::SourceFile;

pub fn ast_to_string(source: &SourceFile) -> String {
    let mut indent = 0;
    let mut out = String::new();

    for event in source.syntax().preorder() {
        match event {
            WalkEvent::Enter(node) => {
                writeln!(out, "{:indent$}{:?}", "", node, indent = indent).unwrap();
                indent += 2;
            }
            WalkEvent::Leave(_) => indent -= 2,
        }
    }

    out
}
