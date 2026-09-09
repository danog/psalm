//! PHP tokenizer (`token_get_all` / `PhpToken::tokenize`), a port of the Zend language scanner
//! for PHP 8.4 semantics. Produces (id, text, line, pos) tuples; single-character tokens use
//! their ASCII code as id.

use crate::consts::*;
use crate::string::Str;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Initial,
    Scripting,
    LookingForProperty,
    DoubleQuotes,
    Backquote,
    Heredoc,
    Nowdoc,
    VarOffset,
    LookingForVarname,
    EndHeredoc,
}

struct Lexer<'a> {
    s: &'a [u8],
    pos: usize,
    line: i64,
    state_stack: Vec<State>,
    tokens: Vec<(i64, Str, i64, i64)>,
    heredoc_label: Vec<u8>,
    heredoc_indent: usize,
    halted: bool,
    halt_line: i64,
}

fn is_label_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c >= 0x80
}
fn is_label_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80
}
fn is_ws(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r')
}

fn keyword(word: &[u8]) -> Option<i64> {
    let lc = word.to_ascii_lowercase();
    Some(match lc.as_slice() {
        b"exit" | b"die" => T_EXIT,
        b"fn" => T_FN,
        b"function" => T_FUNCTION,
        b"const" => T_CONST,
        b"return" => T_RETURN,
        b"yield" => T_YIELD,
        b"try" => T_TRY,
        b"catch" => T_CATCH,
        b"finally" => T_FINALLY,
        b"throw" => T_THROW,
        b"if" => T_IF,
        b"elseif" => T_ELSEIF,
        b"endif" => T_ENDIF,
        b"else" => T_ELSE,
        b"while" => T_WHILE,
        b"endwhile" => T_ENDWHILE,
        b"do" => T_DO,
        b"for" => T_FOR,
        b"endfor" => T_ENDFOR,
        b"foreach" => T_FOREACH,
        b"endforeach" => T_ENDFOREACH,
        b"declare" => T_DECLARE,
        b"enddeclare" => T_ENDDECLARE,
        b"instanceof" => T_INSTANCEOF,
        b"as" => T_AS,
        b"switch" => T_SWITCH,
        b"match" => T_MATCH,
        b"endswitch" => T_ENDSWITCH,
        b"case" => T_CASE,
        b"default" => T_DEFAULT,
        b"break" => T_BREAK,
        b"continue" => T_CONTINUE,
        b"goto" => T_GOTO,
        b"echo" => T_ECHO,
        b"print" => T_PRINT,
        b"class" => T_CLASS,
        b"interface" => T_INTERFACE,
        b"trait" => T_TRAIT,
        b"extends" => T_EXTENDS,
        b"implements" => T_IMPLEMENTS,
        b"new" => T_NEW,
        b"clone" => T_CLONE,
        b"var" => T_VAR,
        b"eval" => T_EVAL,
        b"include" => T_INCLUDE,
        b"include_once" => T_INCLUDE_ONCE,
        b"require" => T_REQUIRE,
        b"require_once" => T_REQUIRE_ONCE,
        b"namespace" => T_NAMESPACE,
        b"use" => T_USE,
        b"insteadof" => T_INSTEADOF,
        b"global" => T_GLOBAL,
        b"isset" => T_ISSET,
        b"empty" => T_EMPTY,
        b"__halt_compiler" => T_HALT_COMPILER,
        b"static" => T_STATIC,
        b"abstract" => T_ABSTRACT,
        b"final" => T_FINAL,
        b"private" => T_PRIVATE,
        b"protected" => T_PROTECTED,
        b"public" => T_PUBLIC,
        b"readonly" => T_READONLY,
        b"unset" => T_UNSET,
        b"list" => T_LIST,
        b"array" => T_ARRAY,
        b"callable" => T_CALLABLE,
        b"__class__" => T_CLASS_C,
        b"__trait__" => T_TRAIT_C,
        b"__function__" => T_FUNC_C,
        b"__method__" => T_METHOD_C,
        b"__property__" => T_PROPERTY_C,
        b"__line__" => T_LINE,
        b"__file__" => T_FILE,
        b"__dir__" => T_DIR,
        b"__namespace__" => T_NS_C,
        b"and" => T_LOGICAL_AND,
        b"or" => T_LOGICAL_OR,
        b"xor" => T_LOGICAL_XOR,
        _ => return None,
    })
}

/// Multi-character operators, longest first.
const OPERATORS: &[(&[u8], i64)] = &[
    (b"<<=", T_SL_EQUAL),
    (b">>=", T_SR_EQUAL),
    (b"**=", T_POW_EQUAL),
    (b"...", T_ELLIPSIS),
    (b"<=>", T_SPACESHIP),
    (b"===", T_IS_IDENTICAL),
    (b"!==", T_IS_NOT_IDENTICAL),
    (b"??=", T_COALESCE_EQUAL),
    (b"?->", T_NULLSAFE_OBJECT_OPERATOR),
    (b"++", T_INC),
    (b"--", T_DEC),
    (b"->", T_OBJECT_OPERATOR),
    (b"=>", T_DOUBLE_ARROW),
    (b"::", T_PAAMAYIM_NEKUDOTAYIM),
    (b"==", T_IS_EQUAL),
    (b"!=", T_IS_NOT_EQUAL),
    (b"<>", T_IS_NOT_EQUAL),
    (b"<=", T_IS_SMALLER_OR_EQUAL),
    (b">=", T_IS_GREATER_OR_EQUAL),
    (b"+=", T_PLUS_EQUAL),
    (b"-=", T_MINUS_EQUAL),
    (b"*=", T_MUL_EQUAL),
    (b"/=", T_DIV_EQUAL),
    (b".=", T_CONCAT_EQUAL),
    (b"%=", T_MOD_EQUAL),
    (b"&=", T_AND_EQUAL),
    (b"|=", T_OR_EQUAL),
    (b"^=", T_XOR_EQUAL),
    (b"&&", T_BOOLEAN_AND),
    (b"||", T_BOOLEAN_OR),
    (b"<<", T_SL),
    (b">>", T_SR),
    (b"**", T_POW),
    (b"??", T_COALESCE),
    (b"#[", T_ATTRIBUTE),
    (b"|>", T_PIPE),
];

impl<'a> Lexer<'a> {
    fn state(&self) -> State {
        *self.state_stack.last().unwrap()
    }
    fn push_state(&mut self, s: State) {
        self.state_stack.push(s);
    }
    fn pop_state(&mut self) {
        if self.state_stack.len() > 1 {
            self.state_stack.pop();
        }
    }
    fn set_state(&mut self, s: State) {
        *self.state_stack.last_mut().unwrap() = s;
    }
    fn peek(&self, off: usize) -> u8 {
        self.s.get(self.pos + off).copied().unwrap_or(0)
    }
    fn rest(&self) -> &'a [u8] {
        &self.s[self.pos..]
    }
    fn starts_with_ci(&self, w: &[u8]) -> bool {
        let r = self.rest();
        r.len() >= w.len() && r[..w.len()].eq_ignore_ascii_case(w)
    }
    fn count_lines(&self, text: &[u8]) -> i64 {
        let mut n = 0;
        let mut i = 0;
        while i < text.len() {
            if text[i] == b'\r' {
                n += 1;
                if i + 1 < text.len() && text[i + 1] == b'\n' {
                    i += 1;
                }
            } else if text[i] == b'\n' {
                n += 1;
            }
            i += 1;
        }
        n
    }
    /// Emit a token spanning [start, end).
    fn emit(&mut self, id: i64, start: usize, end: usize) {
        let text = &self.s[start..end];
        self.tokens.push((id, Str::from_bytes(text), self.line, start as i64));
        self.line += self.count_lines(text);
        self.pos = end;
    }
    fn emit_len(&mut self, id: i64, len: usize) {
        let start = self.pos;
        self.emit(id, start, start + len);
    }
    fn emit_char(&mut self) {
        let c = self.peek(0) as i64;
        self.emit_len(c, 1);
    }

    fn run(&mut self) {
        while self.pos < self.s.len() {
            if self.halted {
                let start = self.pos;
                let end = self.s.len();
                let line = self.halt_line;
                self.tokens.push((T_INLINE_HTML, Str::from_bytes(&self.s[start..end]), line, start as i64));
                self.pos = end;
                break;
            }
            match self.state() {
                State::Initial => self.initial(),
                State::Scripting => self.scripting(),
                State::LookingForProperty => self.looking_for_property(),
                State::DoubleQuotes => self.double_quotes(),
                State::Backquote => self.backquote(),
                State::Heredoc => self.heredoc_body(false),
                State::Nowdoc => self.heredoc_body(true),
                State::VarOffset => self.var_offset(),
                State::LookingForVarname => self.looking_for_varname(),
                State::EndHeredoc => self.end_heredoc(),
            }
        }
    }

    fn initial(&mut self) {
        let start = self.pos;
        let mut i = self.pos;
        while i < self.s.len() {
            if self.s[i] == b'<' && self.s[i + 1..].starts_with(b"?") {
                let r = &self.s[i..];
                if r.len() >= 5 && r[..5].eq_ignore_ascii_case(b"<?php") {
                    let after = r.get(5).copied();
                    if after.is_none() || matches!(after, Some(b' ' | b'\t' | b'\n' | b'\r')) {
                        break;
                    }
                } else if r.starts_with(b"<?=") {
                    break;
                }
            }
            i += 1;
        }
        if i > start {
            self.emit(T_INLINE_HTML, start, i);
            return;
        }
        // open tag
        let r = self.rest();
        if r.starts_with(b"<?=") {
            self.emit_len(T_OPEN_TAG_WITH_ECHO, 3);
        } else {
            let mut len = 5;
            match r.get(5) {
                Some(b'\r') if r.get(6) == Some(&b'\n') => len = 7,
                Some(b' ' | b'\t' | b'\n' | b'\r') => len = 6,
                _ => {}
            }
            self.emit_len(T_OPEN_TAG, len);
        }
        self.set_state(State::Scripting);
    }

    fn scripting(&mut self) {
        let c = self.peek(0);
        let r = self.rest();

        // whitespace
        if is_ws(c) {
            let mut n = 0;
            while n < r.len() && is_ws(r[n]) {
                n += 1;
            }
            self.emit_len(T_WHITESPACE, n);
            return;
        }
        // close tag
        if r.starts_with(b"?>") {
            let mut len = 2;
            if r.get(2) == Some(&b'\n') {
                len = 3;
            } else if r.get(2) == Some(&b'\r') {
                len = if r.get(3) == Some(&b'\n') { 4 } else { 3 };
            }
            self.emit_len(T_CLOSE_TAG, len);
            self.set_state(State::Initial);
            return;
        }
        // comments
        if c == b'#' || r.starts_with(b"//") {
            if r.starts_with(b"#[") {
                self.emit_len(T_ATTRIBUTE, 2);
                return;
            }
            let mut n = 0;
            while n < r.len() {
                match r[n] {
                    b'\n' => break,
                    b'\r' => break,
                    b'?' if r.get(n + 1) == Some(&b'>') => break,
                    _ => n += 1,
                }
            }
            self.emit_len(T_COMMENT, n);
            return;
        }
        if r.starts_with(b"/*") {
            let is_doc = r.starts_with(b"/**") && r.get(3).map_or(false, |c| is_ws(*c));
            let end = crate::string::find_bytes(r, b"*/", 2).map(|p| p + 2).unwrap_or(r.len());
            self.emit_len(if is_doc { T_DOC_COMMENT } else { T_COMMENT }, end);
            return;
        }
        // variables
        if c == b'$' && is_label_start(self.peek(1)) {
            let mut n = 1;
            while n < r.len() && is_label_char(r[n]) {
                n += 1;
            }
            self.emit_len(T_VARIABLE, n);
            return;
        }
        // numbers
        if c.is_ascii_digit() || (c == b'.' && self.peek(1).is_ascii_digit()) {
            self.number();
            return;
        }
        // strings (with optional binary prefix)
        let bprefix = (c == b'b' || c == b'B') && matches!(self.peek(1), b'\'' | b'"' | b'<');
        if bprefix && self.peek(1) == b'<' && !r[1..].starts_with(b"<<<") {
            // not a heredoc
        } else if bprefix {
            self.pos += 1;
            let saved_start = self.pos - 1;
            let before = self.tokens.len();
            match self.peek(0) {
                b'\'' => self.single_quoted(),
                b'"' => self.double_quoted_start(),
                _ => {
                    if !self.heredoc_start() {
                        self.pos = saved_start;
                        let n = self.scan_name(0);
                        self.emit_len(T_STRING, n);
                        return;
                    }
                }
            }
            // extend the first emitted token backwards to include the prefix
            if self.tokens.len() > before {
                let t = &mut self.tokens[before];
                let mut text = vec![self.s[saved_start]];
                text.extend_from_slice(t.1.as_bytes());
                t.1 = Str::from_vec(text);
                t.3 = saved_start as i64;
            }
            return;
        }
        if c == b'\'' {
            self.single_quoted();
            return;
        }
        if c == b'"' {
            self.double_quoted_start();
            return;
        }
        if c == b'`' {
            self.emit_len(b'`' as i64, 1);
            self.push_state(State::Backquote);
            return;
        }
        if r.starts_with(b"<<<") {
            if self.heredoc_start() {
                return;
            }
        }
        // casts
        if c == b'(' {
            if let Some((id, len)) = self.cast() {
                self.emit_len(id, len);
                return;
            }
        }
        // names
        if c == b'\\' {
            if is_label_start(self.peek(1)) {
                let n = self.scan_name(1);
                self.emit_len(T_NAME_FULLY_QUALIFIED, n);
            } else {
                self.emit_len(T_NS_SEPARATOR, 1);
            }
            return;
        }
        if is_label_start(c) {
            let n = self.scan_name(0);
            let text = &r[..n];
            if text.contains(&b'\\') {
                let first_seg_end = text.iter().position(|&b| b == b'\\').unwrap();
                if text[..first_seg_end].eq_ignore_ascii_case(b"namespace") {
                    self.emit_len(T_NAME_RELATIVE, n);
                } else {
                    self.emit_len(T_NAME_QUALIFIED, n);
                }
                return;
            }
            // asymmetric visibility: public(set) etc.
            if let Some((id, len)) = self.visibility_set(text) {
                self.emit_len(id, len);
                return;
            }
            // `yield from`
            if text.eq_ignore_ascii_case(b"yield") {
                let mut k = n;
                let mut saw_ws = false;
                while k < r.len() && is_ws(r[k]) {
                    k += 1;
                    saw_ws = true;
                }
                if saw_ws && r[k..].len() >= 4 && r[k..k + 4].eq_ignore_ascii_case(b"from") && !r.get(k + 4).map_or(false, |c| is_label_char(*c)) {
                    self.emit_len(T_YIELD_FROM, k + 4);
                    return;
                }
            }
            match keyword(text) {
                Some(T_HALT_COMPILER) => {
                    self.emit_len(T_HALT_COMPILER, n);
                    self.halt_compiler();
                }
                Some(id) => {
                    // `enum` is only a keyword when followed by whitespace and an identifier
                    self.emit_len(id, n);
                }
                None => {
                    if text.eq_ignore_ascii_case(b"enum") && self.enum_follows(n) {
                        self.emit_len(T_ENUM, n);
                    } else {
                        self.emit_len(T_STRING, n);
                    }
                }
            }
            return;
        }
        // operators
        for (op, id) in OPERATORS {
            if r.starts_with(op) {
                let id = *id;
                let len = op.len();
                self.emit_len(id, len);
                if id == T_OBJECT_OPERATOR || id == T_NULLSAFE_OBJECT_OPERATOR {
                    self.push_state(State::LookingForProperty);
                }
                return;
            }
        }
        match c {
            b'{' => {
                self.emit_char();
                self.push_state(State::Scripting);
            }
            b'}' => {
                self.emit_char();
                self.pop_state_if_nested();
            }
            b';' | b':' | b',' | b'.' | b'[' | b']' | b'(' | b')' | b'|' | b'^' | b'&' | b'+' | b'-' | b'/' | b'*' | b'=' | b'%' | b'!'
            | b'~' | b'$' | b'<' | b'>' | b'?' | b'@' => {
                if c == b'&' {
                    // T_AMPERSAND_* variants (PHP 8.1)
                    let mut k = 1;
                    while k < r.len() && is_ws(r[k]) {
                        k += 1;
                    }
                    let followed = r.get(k) == Some(&b'$') || r[k..].starts_with(b"...");
                    self.emit_len(if followed { T_AMPERSAND_FOLLOWED_BY_VAR_OR_VARARG } else { T_AMPERSAND_NOT_FOLLOWED_BY_VAR_OR_VARARG }, 1);
                } else {
                    self.emit_char();
                }
            }
            _ => {
                self.emit_len(T_BAD_CHARACTER, 1);
            }
        }
    }

    fn pop_state_if_nested(&mut self) {
        // `}` closes a nested Scripting state pushed by `{`, `{$`, `${`
        if self.state_stack.len() > 1 {
            self.state_stack.pop();
        }
    }

    fn enum_follows(&self, n: usize) -> bool {
        let r = &self.rest()[n..];
        let mut k = 0;
        while k < r.len() && is_ws(r[k]) {
            k += 1;
        }
        if k == 0 {
            return false;
        }
        // followed by an identifier that is not `extends`/`implements`
        let mut m = k;
        while m < r.len() && is_label_char(r[m]) {
            m += 1;
        }
        if m == k || !is_label_start(r[k]) {
            return false;
        }
        let word = &r[k..m];
        !(word.eq_ignore_ascii_case(b"extends") || word.eq_ignore_ascii_case(b"implements"))
    }

    fn visibility_set(&self, text: &[u8]) -> Option<(i64, usize)> {
        // `public(set)` etc.: the scanner rule allows no whitespace anywhere inside
        let id = if text.eq_ignore_ascii_case(b"public") {
            T_PUBLIC_SET
        } else if text.eq_ignore_ascii_case(b"protected") {
            T_PROTECTED_SET
        } else if text.eq_ignore_ascii_case(b"private") {
            T_PRIVATE_SET
        } else {
            return None;
        };
        let r = self.rest();
        let k = text.len();
        if r.len() < k + 5 || r[k] != b'(' || !r[k + 1..k + 4].eq_ignore_ascii_case(b"set") || r[k + 4] != b')' {
            return None;
        }
        Some((id, k + 5))
    }

    fn scan_name(&self, mut n: usize) -> usize {
        let r = self.rest();
        loop {
            while n < r.len() && is_label_char(r[n]) {
                n += 1;
            }
            if n < r.len() && r[n] == b'\\' && n + 1 < r.len() && is_label_start(r[n + 1]) {
                n += 1;
                continue;
            }
            break;
        }
        n
    }

    fn cast(&self) -> Option<(i64, usize)> {
        let r = self.rest();
        let mut k = 1;
        while k < r.len() && (r[k] == b' ' || r[k] == b'\t') {
            k += 1;
        }
        let start = k;
        while k < r.len() && r[k].is_ascii_alphabetic() {
            k += 1;
        }
        let word = r[start..k].to_ascii_lowercase();
        let id = match word.as_slice() {
            b"int" | b"integer" => T_INT_CAST,
            b"real" | b"double" | b"float" => T_DOUBLE_CAST,
            b"string" | b"binary" => T_STRING_CAST,
            b"array" => T_ARRAY_CAST,
            b"object" => T_OBJECT_CAST,
            b"bool" | b"boolean" => T_BOOL_CAST,
            b"unset" => T_UNSET_CAST,
            b"void" => T_VOID_CAST,
            _ => return None,
        };
        while k < r.len() && (r[k] == b' ' || r[k] == b'\t') {
            k += 1;
        }
        if r.get(k) != Some(&b')') {
            return None;
        }
        Some((id, k + 1))
    }

    fn halt_compiler(&mut self) {
        // consume optional whitespace/comments, `(`, `)`, `;` then halt
        let mut expect = [b'(', b')', b';'].iter();
        let mut want = expect.next().copied();
        while self.pos < self.s.len() {
            let c = self.peek(0);
            if is_ws(c) {
                let r = self.rest();
                let mut n = 0;
                while n < r.len() && is_ws(r[n]) {
                    n += 1;
                }
                self.emit_len(T_WHITESPACE, n);
                continue;
            }
            match want {
                Some(w) if c == w => {
                    let line = self.line;
                    self.emit_char();
                    want = expect.next().copied();
                    if want.is_none() {
                        self.halted = true;
                        self.halt_line = line;
                        return;
                    }
                }
                Some(b';') if self.rest().starts_with(b"?>") => {
                    // a close tag terminates the halt statement too
                    let line = self.line;
                    let r = self.rest();
                    let mut len = 2;
                    if r.get(2) == Some(&b'\n') {
                        len = 3;
                    } else if r.get(2) == Some(&b'\r') {
                        len = if r.get(3) == Some(&b'\n') { 4 } else { 3 };
                    }
                    self.emit_len(T_CLOSE_TAG, len);
                    self.halted = true;
                    self.halt_line = line;
                    return;
                }
                _ => return,
            }
        }
    }

    fn number(&mut self) {
        let r = self.rest();
        let mut n = 0;
        let is_digit_or_us = |c: u8| c.is_ascii_digit() || c == b'_';
        // hex / octal / binary
        if r.len() > 2 && r[0] == b'0' && (r[1] == b'x' || r[1] == b'X') && r[2].is_ascii_hexdigit() {
            n = 2;
            while n < r.len() && (r[n].is_ascii_hexdigit() || r[n] == b'_') {
                n += 1;
            }
            let digits: Vec<u8> = r[2..n].iter().copied().filter(|c| *c != b'_').collect();
            let v = u64::from_str_radix(std::str::from_utf8(&digits).unwrap(), 16);
            let id = match v {
                Ok(v) if v <= i64::MAX as u64 => T_LNUMBER,
                _ => T_DNUMBER,
            };
            self.emit_len(id, n);
            return;
        }
        if r.len() > 2 && r[0] == b'0' && (r[1] == b'b' || r[1] == b'B') && (r[2] == b'0' || r[2] == b'1') {
            n = 2;
            while n < r.len() && (r[n] == b'0' || r[n] == b'1' || r[n] == b'_') {
                n += 1;
            }
            let digits: Vec<u8> = r[2..n].iter().copied().filter(|c| *c != b'_').collect();
            let id = if digits.len() <= 63 || u64::from_str_radix(std::str::from_utf8(&digits).unwrap(), 2).map_or(false, |v| v <= i64::MAX as u64) { T_LNUMBER } else { T_DNUMBER };
            self.emit_len(id, n);
            return;
        }
        if r.len() > 2 && r[0] == b'0' && (r[1] == b'o' || r[1] == b'O') && (b'0'..=b'7').contains(&r[2]) {
            n = 2;
            while n < r.len() && ((b'0'..=b'7').contains(&r[n]) || r[n] == b'_') {
                n += 1;
            }
            let digits: Vec<u8> = r[2..n].iter().copied().filter(|c| *c != b'_').collect();
            let id = match u64::from_str_radix(std::str::from_utf8(&digits).unwrap(), 8) {
                Ok(v) if v <= i64::MAX as u64 => T_LNUMBER,
                _ => T_DNUMBER,
            };
            self.emit_len(id, n);
            return;
        }
        // decimal / float; `_` only between two digits
        let us_ok = |r: &[u8], i: usize| r[i] != b'_' || (i > 0 && r[i - 1].is_ascii_digit() && r.get(i + 1).map_or(false, |c| c.is_ascii_digit()));
        while n < r.len() && is_digit_or_us(r[n]) && us_ok(r, n) {
            n += 1;
        }
        let mut is_float = false;
        if n < r.len() && r[n] == b'.' && (n > 0 || r.get(n + 1).map_or(false, |c| c.is_ascii_digit())) {
            // `1.` is a float, `.5` too
            let mut m = n + 1;
            while m < r.len() && is_digit_or_us(r[m]) && us_ok(r, m) {
                m += 1;
            }
            if m > n + 1 || n > 0 {
                is_float = true;
                n = m;
            }
        }
        if n < r.len() && (r[n] == b'e' || r[n] == b'E') {
            let mut m = n + 1;
            if m < r.len() && (r[m] == b'+' || r[m] == b'-') {
                m += 1;
            }
            if m < r.len() && r[m].is_ascii_digit() {
                while m < r.len() && is_digit_or_us(r[m]) {
                    m += 1;
                }
                is_float = true;
                n = m;
            }
        }
        if is_float {
            self.emit_len(T_DNUMBER, n);
            return;
        }
        let digits: Vec<u8> = r[..n].iter().copied().filter(|c| *c != b'_').collect();
        let text = std::str::from_utf8(&digits).unwrap();
        let id = if digits.len() > 1 && digits[0] == b'0' {
            // legacy octal: strtol stops at the first invalid digit; overflow makes it a float
            let valid: String = text[1..].chars().take_while(|c| ('0'..='7').contains(c)).collect();
            match u64::from_str_radix(&valid, 8) {
                Ok(v) if v > i64::MAX as u64 => T_DNUMBER,
                Err(_) if valid.len() > 22 => T_DNUMBER,
                _ => T_LNUMBER,
            }
        } else {
            match text.parse::<i64>() {
                Ok(_) => T_LNUMBER,
                Err(_) => T_DNUMBER,
            }
        };
        self.emit_len(id, n);
    }

    fn single_quoted(&mut self) {
        let r = self.rest();
        let mut n = 1;
        while n < r.len() {
            match r[n] {
                b'\\' => n += 2,
                b'\'' => {
                    n += 1;
                    self.emit_len(T_CONSTANT_ENCAPSED_STRING, n.min(r.len()));
                    return;
                }
                _ => n += 1,
            }
        }
        // unterminated: PHP emits T_ENCAPSED_AND_WHITESPACE for the rest
        let len = r.len();
        self.emit_len(T_ENCAPSED_AND_WHITESPACE, len);
    }

    /// Does the double-quoted/heredoc body starting at `r[i]` contain interpolation?
    fn has_interpolation(r: &[u8], end: Option<usize>) -> bool {
        let limit = end.unwrap_or(r.len());
        let mut i = 0;
        while i < limit {
            match r[i] {
                b'\\' => i += 2,
                b'$' => {
                    if i + 1 < limit && (is_label_start(r[i + 1]) || r[i + 1] == b'{') {
                        return true;
                    }
                    i += 1;
                }
                b'{' => {
                    if i + 1 < limit && r[i + 1] == b'$' {
                        return true;
                    }
                    i += 1;
                }
                _ => i += 1,
            }
        }
        false
    }

    fn double_quoted_start(&mut self) {
        let r = self.rest();
        // find closing quote
        let mut n = 1;
        let mut closed = None;
        while n < r.len() {
            match r[n] {
                b'\\' => n += 2,
                b'"' => {
                    closed = Some(n);
                    break;
                }
                _ => n += 1,
            }
        }
        match closed {
            Some(end) if !Self::has_interpolation(&r[1..end], None) => {
                self.emit_len(T_CONSTANT_ENCAPSED_STRING, end + 1);
            }
            _ => {
                self.emit_len(b'"' as i64, 1);
                self.push_state(State::DoubleQuotes);
            }
        }
    }

    /// Scan encapsed content up to the next interpolation or terminator; returns length.
    fn encapsed_run(&self, terminator: Option<u8>) -> usize {
        let r = self.rest();
        let mut i = 0;
        while i < r.len() {
            let c = r[i];
            if let Some(t) = terminator {
                if c == t {
                    break;
                }
            }
            match c {
                b'\\' => {
                    i += 2;
                }
                b'$' => {
                    if i + 1 < r.len() && (is_label_start(r[i + 1]) || r[i + 1] == b'{') {
                        break;
                    }
                    i += 1;
                }
                b'{' => {
                    if i + 1 < r.len() && r[i + 1] == b'$' {
                        break;
                    }
                    i += 1;
                }
                _ => i += 1,
            }
        }
        i.min(r.len())
    }

    /// Shared handling of `$var`, `${`, `{$` inside interpolated contexts.
    fn interpolation(&mut self) -> bool {
        let r = self.rest();
        if r.starts_with(b"{$") {
            self.emit_len(T_CURLY_OPEN, 1);
            self.push_state(State::Scripting);
            return true;
        }
        if r.starts_with(b"${") {
            self.emit_len(T_DOLLAR_OPEN_CURLY_BRACES, 2);
            self.push_state(State::LookingForVarname);
            return true;
        }
        if r[0] == b'$' && r.len() > 1 && is_label_start(r[1]) {
            let mut n = 1;
            while n < r.len() && is_label_char(r[n]) {
                n += 1;
            }
            self.emit_len(T_VARIABLE, n);
            let r = self.rest();
            if r.starts_with(b"[") {
                self.push_state(State::VarOffset);
            } else if r.starts_with(b"->") && r.get(2).map_or(false, |c| is_label_start(*c)) {
                self.push_state(State::LookingForProperty);
            } else if r.starts_with(b"?->") && r.get(3).map_or(false, |c| is_label_start(*c)) {
                self.push_state(State::LookingForProperty);
            }
            return true;
        }
        false
    }

    fn double_quotes(&mut self) {
        let c = self.peek(0);
        if c == b'"' {
            self.emit_char();
            self.pop_state();
            return;
        }
        if self.interpolation() {
            return;
        }
        let n = self.encapsed_run(Some(b'"'));
        if n == 0 {
            // a lone `$` or `{`
            self.emit_char();
            return;
        }
        self.emit_len(T_ENCAPSED_AND_WHITESPACE, n);
    }

    fn backquote(&mut self) {
        let c = self.peek(0);
        if c == b'`' {
            self.emit_char();
            self.pop_state();
            return;
        }
        if self.interpolation() {
            return;
        }
        let n = self.encapsed_run(Some(b'`'));
        if n == 0 {
            self.emit_char();
            return;
        }
        self.emit_len(T_ENCAPSED_AND_WHITESPACE, n);
    }

    fn looking_for_property(&mut self) {
        let r = self.rest();
        let c = self.peek(0);
        if is_ws(c) {
            let mut n = 0;
            while n < r.len() && is_ws(r[n]) {
                n += 1;
            }
            self.emit_len(T_WHITESPACE, n);
            return;
        }
        if r.starts_with(b"->") {
            self.emit_len(T_OBJECT_OPERATOR, 2);
            return;
        }
        if r.starts_with(b"?->") {
            self.emit_len(T_NULLSAFE_OBJECT_OPERATOR, 3);
            return;
        }
        // comments keep the state: `->/* c */name` is still a property name
        if (c == b'#' && !r.starts_with(b"#[")) || r.starts_with(b"//") {
            let mut n = 0;
            while n < r.len() {
                match r[n] {
                    b'\n' | b'\r' => break,
                    b'?' if r.get(n + 1) == Some(&b'>') => break,
                    _ => n += 1,
                }
            }
            self.emit_len(T_COMMENT, n);
            return;
        }
        if r.starts_with(b"/*") {
            let is_doc = r.starts_with(b"/**") && r.get(3).map_or(false, |c| is_ws(*c));
            let end = crate::string::find_bytes(r, b"*/", 2).map(|p| p + 2).unwrap_or(r.len());
            self.emit_len(if is_doc { T_DOC_COMMENT } else { T_COMMENT }, end);
            return;
        }
        if is_label_start(c) {
            let mut n = 0;
            while n < r.len() && is_label_char(r[n]) {
                n += 1;
            }
            self.emit_len(T_STRING, n);
            self.pop_state();
            return;
        }
        self.pop_state();
    }

    fn var_offset(&mut self) {
        let r = self.rest();
        let c = self.peek(0);
        if c == b'[' {
            self.emit_char();
            return;
        }
        if c == b']' {
            self.emit_char();
            self.pop_state();
            return;
        }
        if c.is_ascii_digit() {
            let mut n = 0;
            if r.len() > n + 2 && r[n] == b'0' && (r[n + 1] == b'x' || r[n + 1] == b'X') {
                n += 2;
                while n < r.len() && (r[n].is_ascii_hexdigit() || r[n] == b'_') {
                    n += 1;
                }
            } else if r.len() > n + 2 && r[n] == b'0' && (r[n + 1] == b'b' || r[n + 1] == b'B') {
                n += 2;
                while n < r.len() && (r[n] == b'0' || r[n] == b'1' || r[n] == b'_') {
                    n += 1;
                }
            } else {
                while n < r.len() && (r[n].is_ascii_digit() || r[n] == b'_') {
                    n += 1;
                }
            }
            self.emit_len(T_NUM_STRING, n);
            return;
        }
        if c == b'$' && is_label_start(self.peek(1)) {
            let mut n = 1;
            while n < r.len() && is_label_char(r[n]) {
                n += 1;
            }
            self.emit_len(T_VARIABLE, n);
            return;
        }
        if is_label_start(c) {
            let mut n = 0;
            while n < r.len() && is_label_char(r[n]) {
                n += 1;
            }
            self.emit_len(T_STRING, n);
            return;
        }
        if r.starts_with(b"->") {
            self.emit_len(T_OBJECT_OPERATOR, 2);
            return;
        }
        if r.starts_with(b"?->") {
            self.emit_len(T_NULLSAFE_OBJECT_OPERATOR, 3);
            return;
        }
        if matches!(c, b' ' | b'\n' | b'\r' | b'\t' | b'\\' | b'\'' | b'#') {
            // invalid in offset: an empty encapsed token, then back to the string state
            let start = self.pos;
            self.tokens.push((T_ENCAPSED_AND_WHITESPACE, Str::empty(), self.line, start as i64));
            self.pop_state();
            return;
        }
        if matches!(c, b';' | b':' | b',' | b'.' | b'|' | b'^' | b'&' | b'+' | b'-' | b'/' | b'*' | b'=' | b'%' | b'!' | b'~' | b'$' | b'<' | b'>' | b'?' | b'@' | b'{' | b'}' | b'"' | b'`') {
            self.emit_char();
            return;
        }
        self.emit_len(T_BAD_CHARACTER, 1);
    }

    fn looking_for_varname(&mut self) {
        let r = self.rest();
        let c = self.peek(0);
        if is_label_start(c) {
            let mut n = 0;
            while n < r.len() && is_label_char(r[n]) {
                n += 1;
            }
            if r.get(n) == Some(&b'[') || r.get(n) == Some(&b'}') {
                self.emit_len(T_STRING_VARNAME, n);
                self.set_state(State::Scripting);
                return;
            }
        }
        self.set_state(State::Scripting);
    }

    fn heredoc_start(&mut self) -> bool {
        let r = self.rest();
        let mut k = 3;
        while k < r.len() && (r[k] == b' ' || r[k] == b'\t') {
            k += 1;
        }
        let quote = match r.get(k) {
            Some(b'\'') => Some(b'\''),
            Some(b'"') => Some(b'"'),
            _ => None,
        };
        if quote.is_some() {
            k += 1;
        }
        let ls = k;
        if !r.get(k).map_or(false, |c| is_label_start(*c)) {
            return false;
        }
        while k < r.len() && is_label_char(r[k]) {
            k += 1;
        }
        let label = r[ls..k].to_vec();
        if let Some(q) = quote {
            if r.get(k) != Some(&q) {
                return false;
            }
            k += 1;
        }
        // must be followed by newline
        if r.get(k) == Some(&b'\r') {
            k += 1;
            if r.get(k) == Some(&b'\n') {
                k += 1;
            }
        } else if r.get(k) == Some(&b'\n') {
            k += 1;
        } else {
            return false;
        }
        self.emit_len(T_START_HEREDOC, k);
        self.heredoc_label = label;
        self.push_state(if quote == Some(b'\'') { State::Nowdoc } else { State::Heredoc });
        true
    }

    /// Find the closing label position (start of the label line and its indentation).
    fn find_heredoc_end(&self) -> Option<(usize, usize, usize)> {
        // returns (line_start, label_start, label_end) relative to self.pos
        let r = self.rest();
        let label = &self.heredoc_label;
        let mut i = 0;
        // the closing label may be on the very first line of the body
        loop {
            let line_start = i;
            let mut j = i;
            while j < r.len() && (r[j] == b' ' || r[j] == b'\t') {
                j += 1;
            }
            if r[j..].starts_with(label) && !r.get(j + label.len()).map_or(false, |c| is_label_char(*c)) {
                return Some((line_start, j, j + label.len()));
            }
            // next line
            while i < r.len() && r[i] != b'\n' && r[i] != b'\r' {
                i += 1;
            }
            if i >= r.len() {
                return None;
            }
            if r[i] == b'\r' && r.get(i + 1) == Some(&b'\n') {
                i += 1;
            }
            i += 1;
            if i > r.len() {
                return None;
            }
        }
    }

    fn heredoc_body(&mut self, nowdoc: bool) {
        let end = self.find_heredoc_end();
        let (line_start, label_start, _label_end) = match end {
            Some(e) => e,
            None => {
                let len = self.rest().len();
                if nowdoc {
                    self.emit_len(T_ENCAPSED_AND_WHITESPACE, len);
                    return;
                }
                (len + 1, len + 1, len + 1)
            }
        };
        self.heredoc_indent = label_start - line_start;
        if nowdoc {
            // body up to (excluding) the newline before the closing label
            let mut body_end = line_start;
            if body_end > 0 {
                body_end -= 1;
                if body_end > 0 && self.rest()[body_end] == b'\n' && self.rest()[body_end - 1] == b'\r' {
                    body_end -= 1;
                }
            }
            if body_end > 0 {
                self.emit_len(T_ENCAPSED_AND_WHITESPACE, body_end);
            }
            self.set_state(State::EndHeredoc);
            return;
        }
        // heredoc: interpolated content up to the closing label line
        if line_start == 0 {
            self.set_state(State::EndHeredoc);
            return;
        }
        let r = self.rest();
        let c = r[0];
        if self.interpolation() {
            return;
        }
        let mut n = self.encapsed_run(None);
        let _ = c;
        // don't run past the closing label line
        let mut body_end = line_start.min(r.len());
        if line_start <= r.len() && body_end > 0 {
            body_end -= 1;
            if body_end > 0 && r[body_end] == b'\n' && r[body_end - 1] == b'\r' {
                body_end -= 1;
            }
        }
        if n > body_end {
            n = body_end;
        }
        if n == 0 {
            if body_end == 0 {
                self.set_state(State::EndHeredoc);
                return;
            }
            self.emit_char();
            return;
        }
        self.emit_len(T_ENCAPSED_AND_WHITESPACE, n);
        if self.pos >= self.s.len() {
            self.pop_state();
        } else if self.pos >= self.rest_start_of_label_line() {
            self.set_state(State::EndHeredoc);
        }
    }

    /// Absolute offset where the closing-label line starts (recomputed from current pos).
    fn rest_start_of_label_line(&self) -> usize {
        match self.find_heredoc_end() {
            Some((line_start, _, _)) => self.pos + line_start,
            None => usize::MAX,
        }
    }

    fn end_heredoc(&mut self) {
        // consume the newline before the label (part of T_END_HEREDOC? No: PHP's T_END_HEREDOC is just
        // the label with its indentation; the preceding newline stays in the encapsed text)
        let r = self.rest();
        let mut k = 0;
        if r.get(k) == Some(&b'\r') {
            k += 1;
        }
        if r.get(k) == Some(&b'\n') {
            k += 1;
        }
        if k > 0 {
            // the newline belongs to the body text
            self.emit_len(T_ENCAPSED_AND_WHITESPACE, k);
        }
        let r = self.rest();
        let mut j = 0;
        while j < r.len() && (r[j] == b' ' || r[j] == b'\t') {
            j += 1;
        }
        let label_len = self.heredoc_label.len();
        self.emit_len(T_END_HEREDOC, j + label_len);
        self.pop_state();
    }
}

/// Returns (id, text, line, pos) tuples.
pub fn tokenize(code: &Str) -> Vec<(i64, Str, i64, i64)> {
    let mut lx = Lexer {
        s: code.as_bytes(),
        pos: 0,
        line: 1,
        state_stack: vec![State::Initial],
        tokens: Vec::new(),
        heredoc_label: Vec::new(),
        heredoc_indent: 0,
        halted: false,
        halt_line: 1,
    };
    lx.run();
    merge_encapsed(lx.tokens)
}

/// Adjacent T_ENCAPSED_AND_WHITESPACE tokens are merged (as the scanner would produce a single token).
fn merge_encapsed(tokens: Vec<(i64, Str, i64, i64)>) -> Vec<(i64, Str, i64, i64)> {
    let mut out: Vec<(i64, Str, i64, i64)> = Vec::with_capacity(tokens.len());
    for t in tokens {
        if t.0 == T_ENCAPSED_AND_WHITESPACE && !t.1.is_empty() {
            if let Some(last) = out.last_mut() {
                if last.0 == T_ENCAPSED_AND_WHITESPACE && !last.1.is_empty() {
                    last.1.push_bytes(&t.1);
                    continue;
                }
            }
        }
        out.push(t);
    }
    out
}
