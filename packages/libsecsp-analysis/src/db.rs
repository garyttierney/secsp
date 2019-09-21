use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{fs, io};

use rustc_hash::FxHashSet;

use crate::cancellation::{Canceled, CheckCanceled};
use crate::input::{FilesDatabase, SourceRoot};
use crate::syntax::SyntaxDatabase;

#[salsa::database(crate::input::Files, crate::syntax::Syntax)]
#[derive(Debug)]
pub struct AnalysisDatabase {
    runtime: salsa::Runtime<AnalysisDatabase>,
}

impl salsa::ParallelDatabase for AnalysisDatabase {
    fn snapshot(&self) -> salsa::Snapshot<AnalysisDatabase> {
        salsa::Snapshot::new(AnalysisDatabase {
            runtime: self.runtime.snapshot(self),
        })
    }
}

impl salsa::Database for AnalysisDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }

    fn on_propagated_panic(&self) -> ! {
        Canceled::throw()
    }

    fn salsa_event(&self, event: impl Fn() -> salsa::Event<AnalysisDatabase>) {
        match event().kind {
            salsa::EventKind::DidValidateMemoizedValue { .. }
            | salsa::EventKind::WillExecute { .. } => {
                self.check_canceled();
            }
            _ => (),
        }
    }
}

impl Default for AnalysisDatabase {
    fn default() -> Self {
        AnalysisDatabase {
            runtime: salsa::Runtime::default(),
        }
    }
}

impl AnalysisDatabase {
    pub fn from_workspace_root<P>(path: P) -> Result<Self, io::Error>
    where
        P: AsRef<Path>,
    {
        let ws_root_path = path.as_ref().to_path_buf();
        let mut ws_dir_stack: Vec<PathBuf> = vec![ws_root_path.clone()];
        let mut ws_files: Vec<PathBuf> = vec![];

        while !ws_dir_stack.is_empty() {
            let ws_dir = ws_dir_stack.pop().unwrap();

            for entry in fs::read_dir(ws_dir)? {
                let entry = entry?;
                let file_type = entry.file_type()?;
                let file_path = entry.path();

                if file_type.is_dir() {
                    ws_dir_stack.push(file_path);
                } else if file_path.extension().filter(|ext| *ext == "csp").is_some() {
                    ws_files.push(file_path);
                }
            }
        }

        Self::from_files(ws_files)
    }

    pub fn from_files(ws_files: Vec<PathBuf>) -> Result<Self, io::Error> {
        let mut db = AnalysisDatabase::default();
        let mut source_root = FxHashSet::default();

        for ws_file in ws_files {
            let id = db.file_path(ws_file.clone());
            let mut file = fs::File::open(ws_file.clone())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            db.set_file_text(id, Arc::new(contents));
            source_root.insert(id);
        }

        db.set_source_root(Arc::new(SourceRoot(source_root)));
        Ok(db)
    }
}
