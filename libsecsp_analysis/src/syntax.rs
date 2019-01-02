use salsa::Database;

use secsp_syntax::ast;
use secsp_syntax::parser;

use crate::input::{FileId, FilesDatabase};

salsa::query_group! {
    pub trait SyntaxDatabase: FilesDatabase + Database {
        fn source_file(file_id: FileId) -> ast::SourceFileNode {
            type SourceFileQuery;
        }
    }
}

fn source_file(db: &impl SyntaxDatabase, file_id: FileId) -> ast::SourceFileNode {
    parser::parse_file(&*db.file_text(file_id))
}
