extern crate futures;
extern crate jsonrpc_core;
extern crate secsp_analysis;
extern crate serde_json;
extern crate text_unit;
extern crate tokio;
extern crate tower_lsp;

use std::sync::{Arc, Mutex};
use std::{mem, panic};

use futures::future;
use jsonrpc_core::{BoxFuture, Result};
use serde_json::Value;
use tower_lsp::lsp_types::*;
use tower_lsp::{LanguageServer, LspService, Printer, Server};

use secsp_analysis::{Analysis, AnalysisHost, Cancelable};
use std::path::PathBuf;

#[derive(Debug, Default)]
struct CspBackend {
    analysis_host: Arc<Mutex<Option<AnalysisHost>>>,
    source_root: Arc<Option<PathBuf>>,
}

impl CspBackend {
    fn initialize_analysis_host(&self, root: String) {
        let host = AnalysisHost::from_workspace(root);
        let mut host_state = self
            .analysis_host
            .lock()
            .expect("Mutex acquired before initialization");

        mem::replace(&mut *host_state, Some(host));
    }

    fn with_analysis_host<T: 'static, F>(&self, executor: F) -> T
    where
        F: FnOnce(&mut AnalysisHost) -> T,
    {
        let mut host_guard = self.analysis_host.lock().unwrap();

        if let Some(ref mut host) = *host_guard {
            executor(host)
        } else {
            panic!("AnalysisHost requested before initialization")
        }
    }

    fn with_analysis<T: 'static, F>(&self, executor: F) -> BoxFuture<T>
    where
        T: Send,
        F: FnOnce(Analysis) -> Cancelable<T>,
    {
        let analysis = self.with_analysis_host(|host| host.analysis());

        match executor(analysis) {
            Ok(t) => Box::new(future::ok(t)),
            Err(_) => Box::new(future::empty()),
        }
    }
}

impl LanguageServer for CspBackend {
    type ShutdownFuture = BoxFuture<()>;
    type SymbolFuture = BoxFuture<Option<Vec<SymbolInformation>>>;
    type ExecuteFuture = BoxFuture<Option<Value>>;
    type HoverFuture = BoxFuture<Option<Hover>>;
    type HighlightFuture = BoxFuture<Option<Vec<DocumentHighlight>>>;

    fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        self.initialize_analysis_host(params.root_path.unwrap_or_else(|| ".".to_string()));

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Full,
                )),
                hover_provider: Some(true),
                ..ServerCapabilities::default()
            },
        })
    }

    fn initialized(&self, printer: &Printer, _p: InitializedParams) {
        printer.log_message(MessageType::Info, "server initialized!");
    }

    fn shutdown(&self) -> Self::ShutdownFuture {
        Box::new(future::ok(()))
    }

    fn symbol(&self, _: WorkspaceSymbolParams) -> Self::SymbolFuture {
        Box::new(future::ok(None))
    }

    fn execute_command(&self, printer: &Printer, _: ExecuteCommandParams) -> Self::ExecuteFuture {
        printer.log_message(MessageType::Info, "command executed!");
        printer.apply_edit(WorkspaceEdit::default());
        Box::new(future::ok(None))
    }

    fn did_change(&self, _printer: &Printer, change: DidChangeTextDocumentParams) {
        if let Ok(path) = change.text_document.uri.to_file_path() {
            self.with_analysis_host(|host| {
                let content = &change.content_changes;
                host.add_file(path, content[0].text.clone());
            })
        }
    }

    fn hover(&self, params: TextDocumentPositionParams) -> Self::HoverFuture {
        self.with_analysis(|analysis| {
            let path = params.text_document.uri.to_file_path().unwrap();

            let source_id = analysis.file_id(path)?;
            let source = analysis.source_file(source_id)?;
            let node = source.syntax_node();

            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(node.to_string())),
                range: None,
            }))
        })
    }

    fn document_highlight(&self, _: TextDocumentPositionParams) -> Self::HighlightFuture {
        Box::new(future::ok(None))
    }
}

fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(CspBackend::default());
    let handle = service.close_handle();
    let server = Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service);

    tokio::run(handle.run_until_exit(server));
}
