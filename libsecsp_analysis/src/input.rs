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

salsa::query_group! {
    pub trait FilesDatabase: Database {
        /// Text of the file.
        fn file_text(file_id: FileId) -> Arc<String> {
            type FileTextQuery;
            storage input;
        }

        /// Relative path of a file to the analysis workspace.
        fn file_relative_path(file_id: FileId) -> PathBuf {
            type FileRelativePathQuery;
            storage input;
        }

        /// The source root of the analysis workspace.
        fn source_root() -> Arc<SourceRoot> {
            type SourceRootQuery;
            storage input;
        }
    }
}
