use codespan::{FileMap, FileName};

use std::path::PathBuf;
use std::sync::Arc;

use crate::ast::Module;
use crate::diagnostic::DiagnosticBuilder;
use crate::lex::token_tree::TokenTree;
use crate::lex::Tokenizer;
use crate::parse::parser::Parser;
use crate::session::ParseSession;

pub mod expr;
pub mod parser;
pub mod stmt;

#[cfg(test)]
pub(crate) mod parser_test;

pub type ParseResult<T> = Result<T, DiagnosticBuilder>;

pub fn parse_module_from_source<I>(sess: &ParseSession, source: I) -> ParseResult<Module>
where
    I: Into<String>,
{
    let mut parser = parser_from_source(sess, source)?;
    parser.parse_module()
}

pub fn parse_module_from_file<I>(sess: &ParseSession, source: I) -> ParseResult<Module>
where
    I: Into<PathBuf>,
{
    let mut parser = parser_from_file(sess, source)?;
    parser.parse_module()
}

pub fn parser_from_source<I>(sess: &ParseSession, source: I) -> ParseResult<Parser<'_>>
where
    I: Into<String>,
{
    file_map_to_parser(sess, source_to_file_map(sess, source))
}

pub fn parser_from_file<I>(sess: &ParseSession, source: I) -> ParseResult<Parser<'_>>
where
    I: Into<PathBuf>,
{
    file_map_to_parser(sess, path_to_file_map(sess, source))
}

pub fn source_to_file_map<I>(sess: &ParseSession, source: I) -> Arc<FileMap>
where
    I: Into<String>,
{
    sess.add_file_map(FileName::Virtual("unknown".into()), source)
}

pub fn path_to_file_map<I>(sess: &ParseSession, source: I) -> Arc<FileMap>
where
    I: Into<PathBuf>,
{
    sess.add_file_map_from_disk(source)
}

pub fn file_map_to_parser(sess: &ParseSession, file_map: Arc<FileMap>) -> ParseResult<Parser<'_>> {
    Ok(Parser::new(sess, file_map_to_stream(sess, file_map)?))
}

pub fn file_map_to_stream(
    sess: &ParseSession,
    file_map: Arc<FileMap>,
) -> ParseResult<Vec<TokenTree>> {
    Tokenizer::new(sess, &file_map).tokenize()
}
