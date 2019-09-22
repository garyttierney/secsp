#![cfg(target_arch = "wasm32")]

use secsp_analysis::{Analysis, AnalysisHost, AnalysisDatabase};
use secsp_analysis::input::{FilesDatabase, FileId, SourceRoot};

use rustc_hash::FxHashSet;
use wasm_bindgen::prelude::*;
use std::sync::Arc;
use std::path::PathBuf;
use std::str::FromStr;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(
        wasm_logger::Config::new(log::Level::Debug)
            .message_on_new_line()
    );
    log::info!("worker initialized")
}

#[wasm_bindgen]
pub struct SingleFileAnalysis {
    analysis_host: AnalysisHost,
    file_id: FileId,
}

#[wasm_bindgen]
impl SingleFileAnalysis {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut analysis_host = AnalysisHost::default();
        let file_id = analysis_host.add_file(PathBuf::from_str("test.csp").unwrap(), "".to_string());

        Self { analysis_host, file_id }
    }

    pub fn update(&mut self, code: String) {
        let _ = self.analysis_host.add_file(PathBuf::from_str("test.csp").unwrap(), code);
    }
}

pub fn from_single_file(text: String) -> (Analysis, FileId) {
    let mut db = AnalysisDatabase::default();
    let source_root = FxHashSet::default();
    db.set_source_root(Arc::new(SourceRoot(source_root)));

    let mut host = AnalysisHost::new(db);
    let id = host.add_file("fake_path.csp".parse().unwrap(), text);

    (host.analysis(), id)
}
