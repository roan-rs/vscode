use roan_engine::module::Module;
use roan_engine::{Stmt, TextSpan, Token, TokenKind};
use ropey::Rope;
use tower_lsp::lsp_types::{SemanticToken, SemanticTokenType};

pub const LEGEND_TYPE: &[SemanticTokenType] = &[
    SemanticTokenType::FUNCTION,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::STRING,
    SemanticTokenType::COMMENT,
    SemanticTokenType::NUMBER,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::STRUCT,
    SemanticTokenType::TYPE,
];

pub fn semantic_tokens(module: Module, rope: Rope) -> Vec<SemanticToken> {
    let mut result = vec![];

    let mut pre_line = 0;
    let mut pre_start = 0;

    for (i, token) in module.tokens.iter().enumerate() {
        let token = token.clone();
        let token_type = match token.kind {
            TokenKind::String(_) => SemanticTokenType::STRING,
            TokenKind::Comment => SemanticTokenType::COMMENT,
            TokenKind::Integer(_) => SemanticTokenType::NUMBER,
            TokenKind::Float(_) => SemanticTokenType::NUMBER,

            // TODO: simplify
            TokenKind::Identifier => {
                let name = token.span.literal.clone();

                let valid = vec![
                    "bool", "int", "float", "string", "void", "anytype", "char", "object",
                ];

                if valid.contains(&&**&name) {
                    SemanticTokenType::TYPE
                } else if name == "self" {
                    SemanticTokenType::PARAMETER
                } else {
                    let r#type = if let Some(Token { kind, .. }) = module.tokens.get(i + 1) {
                        match kind {
                            TokenKind::LeftParen => Some(SemanticTokenType::FUNCTION),
                            TokenKind::Colon => Some(SemanticTokenType::PARAMETER),
                            _ => None
                        }
                    } else {
                        None
                    };

                    if let Some(r#type) = r#type {
                        r#type
                    } else {
                        match module.tokens.get(i - 1).unwrap().kind {
                            TokenKind::Fn => SemanticTokenType::FUNCTION,
                            TokenKind::Struct | TokenKind::Impl | TokenKind::Trait => SemanticTokenType::STRUCT,
                            _ => SemanticTokenType::VARIABLE,
                        }
                    }
                }
            }

            _ if token.kind.is_keyword() => SemanticTokenType::KEYWORD,
            _ if token.kind.is_operator() => SemanticTokenType::OPERATOR,
            _ if token.kind.is_separator() => SemanticTokenType::OPERATOR,
            _ => SemanticTokenType::VARIABLE,
        };

        let line = rope.try_byte_to_line(token.span.start.index).unwrap();
        let first = rope.try_line_to_char(line).unwrap();
        let start = rope.try_byte_to_char(token.span.start.index).unwrap() - first;
        let delta_line = (line - pre_line) as u32;

        let delta_start = if delta_line == 0 {
            start - pre_start
        } else {
            start
        } as u32;

        result.push(SemanticToken {
            delta_line,
            delta_start,
            length: token.span.length() as u32,
            token_type: LEGEND_TYPE.iter().position(|x| {
                x.clone() == token_type
            }).unwrap() as u32,
            token_modifiers_bitset: 0,
        });
        pre_line = line;
        pre_start = start;
    }

    result
}
