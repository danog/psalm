//! Numeric string parsing and float formatting following Zend semantics.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Num {
    Int(i64),
    Float(f64),
}

impl Num {
    pub fn to_f64(self) -> f64 {
        match self {
            Num::Int(i) => i as f64,
            Num::Float(f) => f,
        }
    }
    pub fn to_i64(self) -> i64 {
        match self {
            Num::Int(i) => i,
            Num::Float(f) => float_to_int(f),
        }
    }
}

#[inline]
pub fn is_ws(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

/// Parse the numeric prefix of `b` per `_is_numeric_string_ex`.
/// Returns the number and whether the whole string (modulo surrounding whitespace) was numeric.
/// Returns None when there is no numeric prefix at all.
pub fn parse_numeric_prefix(b: &[u8]) -> Option<(Num, bool)> {
    let mut i = 0;
    while i < b.len() && is_ws(b[i]) {
        i += 1;
    }
    let start = i;
    if i < b.len() && (b[i] == b'-' || b[i] == b'+') {
        i += 1;
    }
    let digits_start = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let int_digits = i - digits_start;
    let mut is_float = false;
    let mut frac_digits = 0;
    if i < b.len() && b[i] == b'.' {
        let mut j = i + 1;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        frac_digits = j - (i + 1);
        if int_digits > 0 || frac_digits > 0 {
            is_float = true;
            i = j;
        }
    }
    if int_digits == 0 && frac_digits == 0 {
        return None;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'-' || b[j] == b'+') {
            j += 1;
        }
        let exp_start = j;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        if j > exp_start {
            is_float = true;
            i = j;
        }
    }
    let end = i;
    let mut k = end;
    while k < b.len() && is_ws(b[k]) {
        k += 1;
    }
    let whole = k == b.len();
    let text = std::str::from_utf8(&b[start..end]).unwrap_or("0");
    if !is_float {
        if let Ok(v) = text.parse::<i64>() {
            return Some((Num::Int(v), whole));
        }
    }
    let f = text.parse::<f64>().unwrap_or(0.0);
    Some((Num::Float(f), whole))
}

/// Full-string numeric check (is_numeric / comparison semantics).
pub fn parse_numeric(b: &[u8]) -> Option<Num> {
    match parse_numeric_prefix(b) {
        Some((n, true)) => Some(n),
        _ => None,
    }
}

pub fn float_to_int(f: f64) -> i64 {
    if !f.is_finite() {
        return 0;
    }
    if f >= -9223372036854775808.0 && f < 9223372036854775808.0 {
        return f as i64;
    }
    let two64 = 18446744073709551616.0f64;
    let dmod = f % two64;
    let mut dmod = if dmod < 0.0 { dmod + two64 } else { dmod };
    if dmod >= 9223372036854775808.0 {
        dmod -= two64;
    }
    dmod as i64
}

/// (int)"..." semantics.
pub fn str_to_int(b: &[u8]) -> i64 {
    match parse_numeric_prefix(b) {
        Some((n, _)) => n.to_i64(),
        None => 0,
    }
}

/// (float)"..." semantics.
pub fn str_to_float(b: &[u8]) -> f64 {
    match parse_numeric_prefix(b) {
        Some((n, _)) => n.to_f64(),
        None => 0.0,
    }
}

fn dtoa(value: f64, ndigit: Option<usize>) -> (String, i32) {
    let s = match ndigit {
        None => format!("{:e}", value.abs()),
        Some(n) => format!("{:.*e}", n.saturating_sub(1), value.abs()),
    };
    let (mant, exp) = s.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let mut digits: String = mant.chars().filter(|c| *c != '.').collect();
    while digits.len() > 1 && digits.ends_with('0') {
        digits.pop();
    }
    (digits, exp + 1)
}

/// zend_gcvt: format with `ndigit` significant digits (None = shortest, threshold 17).
pub fn gcvt(value: f64, ndigit: Option<usize>, exp_char: char) -> String {
    if value == 0.0 {
        return if value.is_sign_negative() { "-0".to_string() } else { "0".to_string() };
    }
    if value.is_nan() {
        return "NAN".to_string();
    }
    if value.is_infinite() {
        return if value < 0.0 { "-INF".to_string() } else { "INF".to_string() };
    }
    let (digits, decpt) = dtoa(value, ndigit);
    let threshold = ndigit.unwrap_or(17) as i32;
    let mut out = String::new();
    if value < 0.0 {
        out.push('-');
    }
    let dbytes = digits.as_bytes();
    if if decpt < 0 { decpt < -3 } else { decpt > threshold } {
        let mut e = decpt - 1;
        let neg = e < 0;
        if neg {
            e = -e;
        }
        out.push(dbytes[0] as char);
        out.push('.');
        if dbytes.len() > 1 {
            out.push_str(&digits[1..]);
        } else {
            out.push('0');
        }
        out.push(exp_char);
        out.push(if neg { '-' } else { '+' });
        out.push_str(&e.to_string());
    } else if decpt <= 0 {
        out.push('0');
        out.push('.');
        for _ in 0..(-decpt) {
            out.push('0');
        }
        out.push_str(&digits);
    } else {
        let d = decpt as usize;
        for i in 0..d {
            if i < dbytes.len() {
                out.push(dbytes[i] as char);
            } else {
                out.push('0');
            }
        }
        if dbytes.len() > d {
            out.push('.');
            out.push_str(&digits[d..]);
        }
    }
    out
}

/// (string)$float — precision=14.
pub fn float_to_string(f: f64) -> String {
    gcvt(f, Some(14), 'E')
}

/// var_export / json_encode style — serialize_precision=-1 (shortest round trip).
pub fn float_to_string_repr(f: f64) -> String {
    gcvt(f, None, 'E')
}

/// var_export appends ".0" for integral floats.
pub fn float_to_export(f: f64) -> String {
    let s = float_to_string_repr(f);
    if f.is_finite() && !s.contains('.') && !s.contains('E') {
        format!("{}.0", s)
    } else {
        s
    }
}

/// A PHP constant's value: the scalar kinds (the typed form of the runtime constant table).
#[derive(Clone, Debug)]
pub enum Scalar {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(crate::string::Str),
}
impl Scalar {
    pub fn to_mixed(self) -> crate::mixed::Mixed {
        match self {
            Scalar::Null => crate::mixed::Mixed::Null,
            Scalar::Bool(b) => crate::mixed::Mixed::Bool(b),
            Scalar::Int(i) => crate::mixed::Mixed::Int(i),
            Scalar::Float(f) => crate::mixed::Mixed::Float(f),
            Scalar::Str(s) => crate::mixed::Mixed::Str(s),
        }
    }
    pub fn from_mixed(m: crate::mixed::Mixed) -> Option<Scalar> {
        Some(match m {
            crate::mixed::Mixed::Null => Scalar::Null,
            crate::mixed::Mixed::Bool(b) => Scalar::Bool(b),
            crate::mixed::Mixed::Int(i) => Scalar::Int(i),
            crate::mixed::Mixed::Float(f) => Scalar::Float(f),
            crate::mixed::Mixed::Str(s) => Scalar::Str(s),
            _ => return None,
        })
    }
}
impl crate::cast::CastTo<crate::mixed::Mixed> for Scalar {
    fn cast_to(self) -> crate::mixed::Mixed {
        self.to_mixed()
    }
}
impl crate::traits::Identical for Scalar {
    fn identical(&self, other: &Self) -> bool {
        crate::traits::Identical::identical(&self.clone().to_mixed(), &other.clone().to_mixed())
    }
}
impl crate::traits::PhpCmp for Scalar {
    fn php_cmp(&self, other: &Self) -> std::cmp::Ordering {
        crate::traits::PhpCmp::php_cmp(&self.clone().to_mixed(), &other.clone().to_mixed())
    }
}
impl crate::traits::Truthy for Scalar {
    fn truthy(&self) -> bool {
        match self {
            Scalar::Null => false,
            Scalar::Bool(b) => *b,
            Scalar::Int(i) => *i != 0,
            Scalar::Float(f) => *f != 0.0,
            Scalar::Str(s) => crate::traits::Truthy::truthy(s),
        }
    }
}
impl crate::traits::ToStr for Scalar {
    fn to_php_str(&self) -> crate::string::Str {
        crate::traits::ToStr::to_php_str(&self.clone().to_mixed())
    }
}
impl crate::traits::PhpKind for Scalar {
    fn php_kind(&self) -> crate::traits::Kind {
        match self {
            Scalar::Null => crate::traits::Kind::Null,
            Scalar::Bool(_) => crate::traits::Kind::Bool,
            Scalar::Int(_) => crate::traits::Kind::Int,
            Scalar::Float(_) => crate::traits::Kind::Float,
            Scalar::Str(_) => crate::traits::Kind::Str,
        }
    }
}
impl crate::traits::InstanceOfName for Scalar {
    fn php_instance_of(&self, _name: &[u8]) -> bool {
        false
    }
}
impl crate::traits::ToNum for Scalar {
    fn to_php_num(&self) -> Num {
        crate::support::to_num(&self.clone().to_mixed())
    }
}
