#![cfg(target_arch = "wasm32")]

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use rustc_hash::FxHashSet;
use secsp_analysis::input::{FileId, FilesDatabase, SourceRoot};
use secsp_analysis::{Analysis, AnalysisDatabase, AnalysisHost};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug).message_on_new_line());
    log::info!("worker initialized")
}

#[wasm_bindgen]
pub struct SingleFileAnalysis {
    analysis_host: AnalysisHost,
    id2file_map: HashMap<String, FileId>,
}

#[wasm_bindgen]
impl SingleFileAnalysis {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut analysis_host = AnalysisHost::default();

        Self {
            analysis_host,
            id2file_map: Default::default(),
        }
    }

    pub fn create_file(&mut self, id: String, code: String) {
        let internal_id = self
            .analysis_host
            .add_file(PathBuf::from_str(&id).unwrap(), code);

        self.id2file_map.insert(id, internal_id);
    }

    pub fn update(&mut self, id: String, code: String) {
        match self.id2file_map.get(&id) {
            Some(id) => self.analysis_host.update_file(*id, code),
            _ => {}
        }
    }
}
