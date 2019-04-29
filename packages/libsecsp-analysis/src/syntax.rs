use salsa::Database;

use secsp_syntax::ast;

use crate::input::{FileId, FilesDatabase};

#[salsa::query_group(Syntax)]
pub trait SyntaxDatabase: FilesDatabase + Database {
    fn source_file(&self, file_id: FileId) -> ast::TreeArc<ast::SourceFile>;
}

fn source_file(db: &impl SyntaxDatabase, file_id: FileId) -> ast::TreeArc<ast::SourceFile> {
    ast::SourceFile::parse(&*db.file_text(file_id))
}
