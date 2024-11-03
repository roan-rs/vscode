use crate::Backend;
use percent_encoding::percent_decode;
use roan_engine::module::Module;
use roan_engine::source::Source;
use roan_error::error::{get_span_from_err, RoanError};
use ropey::Rope;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, MessageType, Position, Range, TextDocumentItem};

impl Backend {
    pub async fn on_change(&self, doc: TextDocumentItem, source: Source) {
        let mut module = Module::new(source.clone());
        module.set_lex_comments(true);

        self.ropes
            .insert(doc.uri.to_string(), Rope::from(source.content()));

        match module.parse() {
            Ok(..) => {
                self.client.publish_diagnostics(doc.uri.clone(), vec![], Some(doc.version)).await;
            }
            Err(err) => {
                self.client
                    .log_message(MessageType::ERROR, format!("Error: {}", err))
                    .await;

                let mut diag = Diagnostic {
                    range: Default::default(),
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: err.to_string(),
                    ..Default::default()
                };

                if let Some(err) = err.downcast_ref::<RoanError>() {
                    let span = get_span_from_err(err);

                    if let Some(span) = span {
                        let range = Range {
                            start: Position {
                                line: span.start.line,
                                character: span.start.column,
                            },
                            end: Position {
                                line: span.end.line,
                                character: span.end.column,
                            },
                        };

                        diag.range = range;
                    }
                }

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
