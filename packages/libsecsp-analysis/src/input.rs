use std::path::PathBuf;
use std::sync::Arc;

use rustc_hash::FxHashSet;
use salsa::{Database, InternId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub salsa::InternId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceRootId(pub salsa::InternId);

#[derive(Debug, Clone)]
pub struct SourceRoot(pub FxHashSet<FileId>);

impl salsa::InternKey for FileId {
    fn from_intern_id(v: InternId) -> Self {
        FileId(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

impl salsa::InternKey for SourceRootId {
    fn from_intern_id(v: InternId) -> Self {
        Self(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[salsa::query_group(Files)]
pub trait FilesDatabase: Database {
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;

    #[salsa::interned]
    fn file_path(&self, data: PathBuf) -> FileId;

    #[salsa::interned]
    fn source_root_path(&self, data: PathBuf) -> SourceRootId;

    #[salsa::input]
    fn source_root(&self) -> Arc<SourceRoot>;
}
