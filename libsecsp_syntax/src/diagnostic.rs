use codespan::CodeMap;
use codespan_reporting;
use codespan_reporting::termcolor::ColorChoice;
use codespan_reporting::termcolor::StandardStream;
use codespan_reporting::{Diagnostic, Label, LabelStyle, Severity};

use parking_lot::Mutex;
use parking_lot::RwLock;

use std::fmt;
use std::fmt::Debug;
use std::sync::Arc;

use crate::lex::ByteSpan;

pub trait DiagnosticEmitter {
    fn emit(&mut self, code_map: &CodeMap, diagnostic: &Diagnostic);
}

pub struct DiagnosticHandler {
    emitter: Mutex<Box<dyn DiagnosticEmitter>>,
    emitted_diagnostics: Mutex<Vec<Diagnostic>>,
    code_map: Arc<RwLock<CodeMap>>,
}

impl DiagnosticHandler {
    pub fn handle(&self, diagnostic: &Diagnostic) {
        self.emitter.lock().emit(&*self.code_map.read(), diagnostic);
        self.emitted_diagnostics.lock().push(diagnostic.clone());
    }

    pub fn new<E: DiagnosticEmitter + 'static>(code_map: Arc<RwLock<CodeMap>>, emitter: E) -> Self {
        DiagnosticHandler {
            code_map,
            emitter: Mutex::new(Box::from(emitter)),
            emitted_diagnostics: Mutex::new(vec![]),
        }
    }
}

pub struct DiagnosticBuilder {
    handler: Arc<DiagnosticHandler>,
    diagnostic: Diagnostic,
}

impl Debug for DiagnosticBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:#?}", &self.diagnostic)
    }
}

impl DiagnosticBuilder {
    pub fn new<S: Into<String>>(
        handler: Arc<DiagnosticHandler>,
        severity: Severity,
        message: S,
    ) -> Self {
        DiagnosticBuilder {
            handler,
            diagnostic: Diagnostic::new(severity, message),
        }
    }

    pub fn span(mut self, sp: ByteSpan) -> Self {
        self.diagnostic = self
            .diagnostic
            .with_label(Label::new(sp, LabelStyle::Primary));

        self
    }

    pub fn span_err<S>(mut self, sp: ByteSpan, msg: S) -> Self
    where
        S: Into<String>,
    {
        self.diagnostic = self
            .diagnostic
            .with_label(Label::new(sp, LabelStyle::Primary).with_message(msg));

        self
    }

    pub fn emit(&mut self) {
        self.handler.handle(&self.diagnostic)
    }
}

pub struct NoopDiagnosticReporter;

impl DiagnosticEmitter for NoopDiagnosticReporter {
    fn emit(&mut self, _code_map: &CodeMap, _diagnostic: &Diagnostic) {}
}

/// A diagnostic reporter that emits diagnostics to the console, using one of the
/// standard output streams (stdout/stderr).
pub struct ConsoleDiagnosticReporter {
    stream: StandardStream,
}

impl<'a> ConsoleDiagnosticReporter {
    pub fn new() -> Self {
        ConsoleDiagnosticReporter {
            stream: StandardStream::stderr(ColorChoice::Always),
        }
    }
}

impl Default for ConsoleDiagnosticReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticEmitter for ConsoleDiagnosticReporter {
    fn emit(&mut self, code_map: &CodeMap, diagnostic: &Diagnostic) {
        codespan_reporting::emit(&mut self.stream, code_map, &diagnostic)
            .expect("failed to write diagnostic");
    }
}
