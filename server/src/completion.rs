use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{CompletionItem, CompletionParams, CompletionResponse};

pub struct Completion {
    pub params: CompletionParams
}

impl Completion {
    pub fn new(params: CompletionParams) -> Self {
        Self { params }
    }
}

impl Completion {
    pub fn response(&self) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string())
        ])))
    }
}