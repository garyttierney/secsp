use std::path::PathBuf;
use std::sync::Arc;

use rustc_hash::FxHashMap;
use salsa::Database;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub u32);

#[derive(Debug)]
pub struct SourceRoot {
    pub file_map: FxHashMap<PathBuf, FileId>,
}

impl Default for SourceRoot {
    fn default() -> Self {
        SourceRoot {
            file_map: FxHashMap::default(),
        }
    }
}

impl SourceRoot {
    // TODO: This is error-prone.
    pub fn add(&mut self, path: PathBuf) -> FileId {
        let id = FileId(self.file_map.len() as u32);
        self.file_map.insert(path, id);

        id
    }

    pub fn file_ids(&self) -> ::std::collections::hash_map::Values<PathBuf, FileId> {
        self.file_map.values()
    }
}

#[salsa::query_group(Files)]
pub trait FilesDatabase: Database {
    #[salsa::input]
    fn file_text(&self, file_id: FileId) -> Arc<String>;

    #[salsa::input]
    fn file_relative_path(&self, file_id: FileId) -> PathBuf;

    #[salsa::input]
    fn source_root(&self) -> Arc<SourceRoot>;
}
