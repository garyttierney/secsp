use codespan::{CodeMap, FileMap, FileName};
use codespan_reporting::Severity;
use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;
use string_interner::{DefaultStringInterner, Symbol};

use crate::diagnostic::ConsoleDiagnosticReporter;
use crate::diagnostic::DiagnosticBuilder;
use crate::diagnostic::DiagnosticEmitter;
use crate::diagnostic::DiagnosticHandler;

pub struct ParseSession {
    code_map: Arc<RwLock<CodeMap>>,
    diagnostic_handler: Arc<DiagnosticHandler>,
    interner: RwLock<DefaultStringInterner>,
}

impl ParseSession {
    pub fn with_diagnostic_emitter<E>(emitter: E) -> Self
    where
        E: DiagnosticEmitter + 'static,
    {
        let code_map = Arc::new(RwLock::new(CodeMap::new()));
        let diagnostic_handler = Arc::new(DiagnosticHandler::new(code_map.clone(), emitter));
        let interner = RwLock::new(DefaultStringInterner::default());

        ParseSession {
            code_map,
            diagnostic_handler,
            interner,
        }
    }

    pub fn add_file_map_from_disk<P>(&self, path: P) -> Arc<FileMap>
    where
        P: Into<PathBuf>,
    {
        self.code_map
            .write()
            .add_filemap_from_disk(path)
            .expect("@todo: handle error")
    }

    pub fn add_file_map<I>(&self, name: FileName, input: I) -> Arc<FileMap>
    where
        I: Into<String>,
    {
        self.code_map.write().add_filemap(name, input.into())
    }

    pub fn interned_value_equals<S: AsRef<str>>(&self, value: usize, str: S) -> bool {
        self.interner
            .read()
            .resolve(value)
            .map_or(false, |sym| sym == str.as_ref())
    }

    pub fn diagnostic<S: Into<String>>(&self, severity: Severity, msg: S) -> DiagnosticBuilder {
        DiagnosticBuilder::new(self.diagnostic_handler.clone(), severity, msg)
    }

    pub fn intern<S: AsRef<str>>(&self, value: S) -> usize {
        self.interner
            .write()
            .get_or_intern(value.as_ref())
            .to_usize()
    }
}

impl Default for ParseSession {
    fn default() -> Self {
        ParseSession::with_diagnostic_emitter(ConsoleDiagnosticReporter::new())
    }
}
