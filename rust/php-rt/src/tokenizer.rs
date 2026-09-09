//! PHP tokenizer (token_get_all / PhpToken::tokenize). Placeholder until the full lexer lands.

use crate::string::Str;

/// Returns (id, text, line, pos) tuples.
pub fn tokenize(code: &Str) -> Vec<(i64, Str, i64, i64)> {
    let _ = code;
    unimplemented!("PHP tokenizer not yet implemented")
}
