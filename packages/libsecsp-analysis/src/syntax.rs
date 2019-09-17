use salsa::Database;

use secsp_syntax::{ast, Parse};

use crate::input::{FileId, FilesDatabase};

#[salsa::query_group(Syntax)]
pub trait SyntaxDatabase: FilesDatabase + Database {
    fn source_file(&self, file_id: FileId) -> Parse<ast::SourceFile>;
}

fn source_file(db: &impl SyntaxDatabase, file_id: FileId) -> Parse<ast::SourceFile> {
    ast::SourceFile::parse(&*db.file_text(file_id))
}
