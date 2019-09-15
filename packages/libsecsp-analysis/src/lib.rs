extern crate rustc_hash;
extern crate salsa;
extern crate secsp_syntax;

use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use salsa::Database;

use secsp_syntax::{ast, Parse};

use crate::input::FilesDatabase;
use crate::input::SourceRoot;

pub mod input;
pub mod syntax;

#[salsa::database(input::Files, syntax::Syntax)]
pub struct AnalysisDatabase {
    runtime: salsa::Runtime<AnalysisDatabase>,
}

impl salsa::Database for AnalysisDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }
}

impl Default for AnalysisDatabase {
    fn default() -> Self {
        let mut db = AnalysisDatabase {
            runtime: salsa::Runtime::default(),
        };

        let source_root = Arc::new(SourceRoot::default());

        db.set_source_root(source_root);
        db
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
                } else if file_path.ends_with(".csp") {
                    ws_files.push(file_path);
                }
            }
        }

        Self::from_files(ws_files)
    }

    pub fn from_files(ws_files: Vec<PathBuf>) -> Result<Self, io::Error> {
        let mut db = AnalysisDatabase::default();
        let mut source_root = input::SourceRoot::default();

        for ws_file in ws_files {
            let id = source_root.add(ws_file.clone());
            let mut file = fs::File::open(ws_file.clone())?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            db.set_file_text(id, Arc::new(contents));
            db.set_file_relative_path(id, ws_file);
        }

        db.set_source_root(Arc::new(source_root));

        Ok(db)
    }
}

pub struct AnalysisHost {
    db: AnalysisDatabase,
}

impl AnalysisHost {
    pub fn new(db: AnalysisDatabase) -> AnalysisHost {
        AnalysisHost { db }
    }

    pub fn from_workspace<P: AsRef<Path>>(ws: P) -> AnalysisHost {
        let db = AnalysisDatabase::from_workspace_root(ws).expect("unable to initialize database");

        AnalysisHost { db }
    }

    pub fn source_file(&self, file_id: input::FileId) -> Parse<ast::SourceFile> {
        self.db.query(syntax::SourceFileQuery).get(file_id).clone()
    }

    pub fn source_root(&self) -> Arc<input::SourceRoot> {
        self.db.query(input::SourceRootQuery).get(())
    }
}

impl Default for AnalysisHost {
    fn default() -> Self {
        AnalysisHost::new(AnalysisDatabase::default())
    }
}
