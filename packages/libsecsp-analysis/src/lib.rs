extern crate rustc_hash;
extern crate salsa;
extern crate secsp_syntax;

use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use salsa::ParallelDatabase;

use secsp_syntax::{ast, Parse};

use crate::cancellation::{Canceled, CheckCanceled};
use crate::input::{FileId, FilesDatabase, SourceRoot};
use crate::syntax::SyntaxDatabase;

pub mod cancellation;
pub mod db;
pub mod input;
pub mod syntax;

pub use db::AnalysisDatabase;

#[derive(Debug)]
pub struct AnalysisHost {
    db: AnalysisDatabase,
}

#[derive(Debug)]
pub struct Analysis {
    db: salsa::Snapshot<AnalysisDatabase>,
}

pub type Cancelable<T> = Result<T, Canceled>;

impl Analysis {
    pub fn file_id(&self, path: PathBuf) -> Cancelable<FileId> {
        self.with_db(|db| db.file_path(path))
    }

    pub fn source_file(&self, file_id: FileId) -> Cancelable<Parse<ast::SourceFile>> {
        self.with_db(|db| db.source_file(file_id))
    }

    pub fn source_root(&self) -> Cancelable<SourceRoot> {
        self.with_db(|db| (*db.source_root()).clone())
    }

    fn with_db<F: FnOnce(&AnalysisDatabase) -> T + std::panic::UnwindSafe, T>(
        &self,
        f: F,
    ) -> Cancelable<T> {
        self.db.catch_canceled(f)
    }
}

impl AnalysisHost {
    pub fn new(db: AnalysisDatabase) -> AnalysisHost {
        AnalysisHost { db }
    }

    pub fn from_workspace<P: AsRef<Path>>(ws: P) -> AnalysisHost {
        let db = AnalysisDatabase::from_workspace_root(ws).expect("unable to initialize database");

        AnalysisHost { db }
    }

    pub fn add_file(&mut self, path: PathBuf, contents: String) {
        let id = self.db.file_path(path);
        let mut source_root = (*self.db.source_root()).clone();

        source_root.0.insert(id);

        self.db.set_source_root(Arc::new(source_root));
        self.db.set_file_text(id, Arc::new(contents));
    }

    pub fn analysis(&self) -> Analysis {
        Analysis {
            db: self.db.snapshot(),
        }
    }
}

impl Default for AnalysisHost {
    fn default() -> Self {
        AnalysisHost::new(AnalysisDatabase::default())
    }
}
