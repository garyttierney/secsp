use lsp_types::{Hover, HoverContents, MarkedString, TextDocumentPositionParams};

pub fn hover_request(_params: TextDocumentPositionParams) -> Result<Option<Hover>, ()> {
    Ok(Some(Hover {
        contents: HoverContents::Scalar(MarkedString::from_markdown("test".to_owned())),
        range: None,
    }))
}
