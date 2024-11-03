use crate::Backend;
use percent_encoding::percent_decode;
use roan_engine::module::Module;
use roan_engine::source::Source;
use roan_error::error::RoanError;
use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, MessageType, TextDocumentItem};

impl Backend {
    pub async fn on_change(&self, doc: TextDocumentItem, source: Source) {
        let mut module = Module::new(source.clone());
        module.set_lex_comments(true);

        self.ropes
            .insert(doc.uri.to_string(), Rope::from(source.content()));

        match module.parse() {
            Ok(..) => {}
            Err(err) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Error: {}", err))
                    .await;
                let diag = Diagnostic {
                    range: Default::default(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.to_string(),
                    ..Default::default()
                };

                self.client
                    .publish_diagnostics(doc.uri.clone(), vec![diag], Some(doc.version))
                    .await;
            }
        }

        self.modules.insert(doc.uri.to_string(), module);
    }
}

pub fn url_to_path(url: &str) -> String {
    let path = url.replace("file://", "");
    percent_decode(path.as_bytes())
        .decode_utf8_lossy()
        .to_string()
        .strip_prefix('/')
        .expect("dwaaaaa")
        .to_string()
}
