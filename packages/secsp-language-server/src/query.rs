use lsp_types::request::{GotoDefinitionResponse, HoverRequest};
use lsp_types::{Hover, HoverContents, MarkedString, TextDocumentPositionParams};

pub fn hover_request(params: TextDocumentPositionParams) -> Result<Option<Hover>, ()> {
    Ok(Some(Hover {
        contents: HoverContents::Scalar(MarkedString::from_markdown("test".to_owned())),
        range: None,
    }))
}
