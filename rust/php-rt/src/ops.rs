//! Arithmetic and string operators with PHP semantics.

use crate::conv::Num;
use crate::error::RtError;
use crate::string::Str;
use crate::traits::ToStr;

#[inline]
pub fn iadd(a: i64, b: i64) -> i64 {
    a.wrapping_add(b)
}
#[inline]
pub fn isub(a: i64, b: i64) -> i64 {
    a.wrapping_sub(b)
}
#[inline]
pub fn imul(a: i64, b: i64) -> i64 {
    a.wrapping_mul(b)
}
/// `intdiv` / `%` helpers.
pub fn imod(a: i64, b: i64) -> i64 {
    if b == 0 {
        panic!("Uncaught exception: DivisionByZeroError: Modulo by zero");
    }
    if b == -1 {
        return 0;
    }
    a % b
}
pub fn intdiv(a: i64, b: i64) -> i64 {
    if b == 0 {
        panic!("Uncaught exception: DivisionByZeroError: Division by zero");
    }
    if a == i64::MIN && b == -1 {
        panic!("Uncaught exception: ArithmeticError: Division of PHP_INT_MIN by -1 is not an integer");
    }
    a / b
}
pub fn div(a: Num, b: Num) -> Num {
    let bf = b.to_f64();
    if bf == 0.0 {
        panic!("Uncaught exception: DivisionByZeroError: Division by zero");
    }
    if let (Num::Int(x), Num::Int(y)) = (a, b) {
        if y != 0 && x % y == 0 && !(x == i64::MIN && y == -1) {
            return Num::Int(x / y);
        }
    }
    Num::Float(a.to_f64() / bf)
}
pub fn div_f(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Uncaught exception: DivisionByZeroError: Division by zero");
    }
    a / b
}
pub fn div_i(a: i64, b: i64) -> i64 {
    match div(Num::Int(a), Num::Int(b)) {
        Num::Int(i) => i,
        Num::Float(f) => f as i64,
    }
}
pub fn pow_i(a: i64, b: i64) -> Num {
    if b >= 0 {
        if let Some(r) = a.checked_pow(b as u32) {
            return Num::Int(r);
        }
    }
    Num::Float((a as f64).powf(b as f64))
}
pub fn pow_f(a: f64, b: f64) -> f64 {
    a.powf(b)
}
pub fn fmod(a: f64, b: f64) -> f64 {
    a % b
}

// ---------------------------------------------------------------- strings

/// `$a . $b`
#[inline]
pub fn concat<A: ToStr, B: ToStr>(a: A, b: B) -> Str {
    let l = a.to_php_str();
    let r = b.to_php_str();
    // one exact-size allocation: the left operand is usually a shared handle (a variable's clone), so
    // pushing into it would first copy it and then grow it
    let (lb, rb) = (l.as_bytes(), r.as_bytes());
    if rb.is_empty() {
        return l;
    }
    if lb.is_empty() {
        return r;
    }
    let mut out = Vec::with_capacity(lb.len() + rb.len());
    out.extend_from_slice(lb);
    out.extend_from_slice(rb);
    Str::from_vec(out)
}
/// `$a .= $b`
#[inline]
pub fn append<B: ToStr>(a: &mut Str, b: B) {
    let r = b.to_php_str();
    a.push_bytes(r.as_bytes());
}
/// Concatenate many parts (string interpolation).
pub fn concat_all(parts: &[&dyn ToStr]) -> Str {
    let mut out = Vec::new();
    for p in parts {
        out.extend_from_slice(p.to_php_str().as_bytes());
    }
    Str::from_vec(out)
}
#[macro_export]
macro_rules! cat {
    ($($e:expr),+ $(,)?) => { $crate::ops::concat_all(&[$(&$e as &dyn $crate::traits::ToStr),+]) };
}

/// `$s[$i]` on strings.
/// Every single-byte string, so that `$s[$i]` never allocates.
static BYTE_TABLE: [u8; 256] = {
    let mut t = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        t[i] = i as u8;
        i += 1;
    }
    t
};
#[inline]
pub fn single_byte_str(c: u8) -> Str {
    Str::from_static_bytes(&BYTE_TABLE[c as usize..c as usize + 1])
}
pub fn str_index(s: &Str, i: i64) -> Str {
    let b = s.as_bytes();
    let idx = if i < 0 { b.len() as i64 + i } else { i };
    if idx < 0 || idx as usize >= b.len() {
        return Str::empty();
    }
    single_byte_str(b[idx as usize])
}
pub fn str_index_byte(s: &[u8], i: i64) -> Option<u8> {
    let idx = if i < 0 { s.len() as i64 + i } else { i };
    if idx < 0 || idx as usize >= s.len() {
        None
    } else {
        Some(s[idx as usize])
    }
}
/// `$s[$i] = $c`
pub fn str_set_index(s: &mut Str, i: i64, c: &[u8]) {
    let len = s.len() as i64;
    let idx = if i < 0 { len + i } else { i };
    if idx < 0 {
        return;
    }
    s.set_index(idx as usize, c.first().copied().unwrap_or(b' '));
}

/// PHP string increment (`$s++`): "a" -> "b", "Az" -> "Ba", "zz" -> "aaa".
pub fn str_increment(s: &Str) -> Str {
    let mut b = s.to_vec();
    if b.is_empty() {
        return Str::from_static("1");
    }
    let mut i = b.len();
    loop {
        if i == 0 {
            let first = b[0];
            let prefix = if first.is_ascii_digit() { b'1' } else if first.is_ascii_uppercase() { b'A' } else { b'a' };
            b.insert(0, prefix);
            break;
        }
        i -= 1;
        let c = b[i];
        match c {
            b'z' => b[i] = b'a',
            b'Z' => b[i] = b'A',
            b'9' => b[i] = b'0',
            _ if c.is_ascii_alphanumeric() => {
                b[i] = c + 1;
                break;
            }
            _ => break,
        }
    }
    Str::from_vec(b)
}
