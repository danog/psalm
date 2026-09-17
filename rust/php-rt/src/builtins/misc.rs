//! Environment, error handling, class/function introspection and assorted builtins.

use crate::containers::DynCallable;
use crate::error::RtError;
use crate::key::ArrayKey;
use crate::list::List;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;
use crate::traits::*;
use std::cell::RefCell;
use crate::{FastMap, fast_map};

thread_local! {
    static INI: RefCell<FastMap<Vec<u8>, Str>> = RefCell::new(fast_map());
    static ERROR_LEVEL: RefCell<i64> = RefCell::new(32767);
    static LAST_ERROR: RefCell<Option<Map<ArrayKey, Mixed>>> = RefCell::new(None);
    static INCLUDED: RefCell<Vec<Str>> = RefCell::new(Vec::new());
}

pub fn getenv(name: &Str) -> Option<Str> {
    std::env::var_os(&*name.to_string_lossy()).map(|v| Str::from_string(v.to_string_lossy().into_owned()))
}
pub fn putenv(assignment: &Str) -> bool {
    let s = assignment.to_string_lossy().into_owned();
    match s.split_once('=') {
        Some((k, v)) => {
            // SAFETY: single-threaded runtime
            unsafe { std::env::set_var(k, v) };
            true
        }
        None => {
            unsafe { std::env::remove_var(&s) };
            true
        }
    }
}
pub fn ini_set(name: &Str, value: &Str) -> Option<Str> {
    INI.with(|i| i.borrow_mut().insert(name.to_vec(), value.clone()))
}
pub fn ini_get(name: &Str) -> Option<Str> {
    INI.with(|i| i.borrow().get(name.as_bytes()).cloned()).or_else(|| match name.as_bytes() {
        b"memory_limit" => Some(Str::from_static("-1")),
        b"xdebug.scream" => None,
        b"precision" => Some(Str::from_static("14")),
        b"zend.assertions" => Some(Str::from_static("1")),
        _ => None,
    })
}
pub fn set_error_handler(_h: &Mixed) {}
pub fn restore_error_handler() -> bool {
    true
}
pub fn set_exception_handler(_h: &Mixed) {}
pub fn error_reporting(level: Option<i64>) -> i64 {
    ERROR_LEVEL.with(|l| {
        let old = *l.borrow();
        if let Some(n) = level {
            *l.borrow_mut() = n;
        }
        old
    })
}
pub fn error_log(msg: &Str) -> bool {
    crate::output::eprint(msg);
    crate::output::eprint(b"\n");
    true
}
pub fn trigger_error(msg: &Str, level: i64) -> Result<bool, RtError> {
    if level == 256 {
        return Err(RtError::new("ErrorException", msg.clone()));
    }
    crate::output::eprint(msg);
    crate::output::eprint(b"\n");
    Ok(true)
}
pub fn error_get_last() -> Option<Mixed> {
    LAST_ERROR.with(|e| e.borrow().clone().map(Mixed::Arr))
}
pub fn gc_collect_cycles() -> i64 {
    0
}
pub fn gc_disable() {}
pub fn gc_enable() {}
pub fn memory_get_usage(_real: bool) -> i64 {
    0
}
pub fn memory_get_peak_usage(_real: bool) -> i64 {
    0
}
pub fn usleep(us: i64) {
    std::thread::sleep(std::time::Duration::from_micros(us.max(0) as u64));
}
pub fn sleep(s: i64) -> i64 {
    std::thread::sleep(std::time::Duration::from_secs(s.max(0) as u64));
    0
}
pub fn extension_loaded(name: &Str) -> bool {
    matches!(name.as_bytes(), b"json" | b"tokenizer" | b"mbstring" | b"ctype" | b"pcre" | b"spl" | b"simplexml" | b"dom" | b"libxml" | b"filter" | b"hash" | b"random")
}
pub fn phpversion(ext: Option<&Str>) -> Option<Str> {
    match ext {
        None => Some(crate::consts::PHP_VERSION),
        Some(e) if extension_loaded(e) => Some(crate::consts::PHP_VERSION),
        _ => None,
    }
}
pub fn php_sapi_name() -> Str {
    Str::from_static("cli")
}
pub fn spl_autoload_register(_f: Option<&Mixed>) -> bool {
    true
}
pub fn setlocale(_cat: i64, _loc: &Str) -> Option<Str> {
    Some(Str::from_static("C"))
}
pub fn date(format: &Str, ts: Option<i64>) -> Str {
    let secs = ts.unwrap_or_else(crate::builtins::math::time);
    let days = secs.div_euclid(86400);
    let rem = secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    let (h, mi, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let mut out = String::new();
    for c in format.to_string_lossy().chars() {
        match c {
            'Y' => out.push_str(&format!("{:04}", y)),
            'm' => out.push_str(&format!("{:02}", m)),
            'd' => out.push_str(&format!("{:02}", d)),
            'H' => out.push_str(&format!("{:02}", h)),
            'i' => out.push_str(&format!("{:02}", mi)),
            's' => out.push_str(&format!("{:02}", s)),
            'U' => out.push_str(&secs.to_string()),
            'D' => out.push_str(["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"][days.rem_euclid(7) as usize]),
            'N' => out.push_str(&((days.rem_euclid(7) + 3) % 7 + 1).to_string()),
            other => out.push(other),
        }
    }
    Str::from_string(out)
}
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}
pub fn strtotime(_s: &Str) -> Option<i64> {
    None
}
pub fn checkdate(m: i64, d: i64, y: i64) -> bool {
    m >= 1 && m <= 12 && d >= 1 && d <= 31 && y >= 1
}
pub fn debug_print_backtrace() {}
pub fn debug_zval_refcount(_m: &Mixed) -> i64 {
    1
}
pub fn cli_set_process_title(_t: &Str) -> bool {
    true
}
pub fn getmypid() -> i64 {
    std::process::id() as i64
}
pub fn gethostname() -> Str {
    Str::from_static("localhost")
}
pub fn get_include_path() -> Str {
    Str::from_static(".")
}
pub fn get_included_files() -> List<Str> {
    INCLUDED.with(|i| i.borrow().clone().into())
}
pub fn get_loaded_extensions() -> List<Str> {
    crate::list![Str::from_static("Core"), Str::from_static("json"), Str::from_static("tokenizer"), Str::from_static("mbstring"), Str::from_static("ctype"), Str::from_static("pcre"), Str::from_static("SPL")]
}
pub fn get_defined_constants(categorize: bool) -> Map<ArrayKey, Mixed> {
    // the compiled program's constants: PHP's builtin table plus the runtime's own (PHP_EOL, PSALM_COMPILED, ...)
    let mut all: Map<ArrayKey, Mixed> = Map::new();
    for (name, value) in crate::php_constants::PHP_CONSTANTS {
        all.insert(ArrayKey::Str(Str::from(*name)), value());
    }
    for name in ["PHP_EOL", "PHP_VERSION", "PHP_VERSION_ID", "PHP_INT_MAX", "PHP_INT_MIN", "PHP_INT_SIZE", "PHP_OS", "PHP_OS_FAMILY", "DIRECTORY_SEPARATOR", "E_ALL", "E_STRICT", "PSALM_COMPILED", "PSALM_VERSION", "PHP_PARSER_VERSION"] {
        if let Some(v) = crate::consts::builtin_value(name.as_bytes()) {
            all.insert(ArrayKey::Str(Str::from(name.as_bytes())), v);
        }
    }
    if categorize {
        let mut out: Map<ArrayKey, Mixed> = Map::new();
        out.insert(ArrayKey::Str(Str::from_static("Core")), Mixed::Arr(all));
        return out;
    }
    all
}
/// Names of the functions the runtime implements natively (see `builtin_callable` and the eval call table).
pub const BUILTIN_FUNCTION_NAMES: &[&str] = &[
    "strtolower", "strtoupper", "ucfirst", "lcfirst", "trim", "strval", "strlen", "intval", "is_string", "is_int",
    "is_array", "is_null", "is_numeric", "is_object", "is_bool", "is_float", "is_scalar", "strcmp", "strcasecmp",
    "strnatcmp", "strnatcasecmp", "ucwords", "ltrim", "rtrim", "md5", "floatval", "boolval", "count", "array_values",
    "array_keys", "array_unique", "array_merge", "preg_match", "array_map", "array_filter", "str_contains",
    "str_starts_with", "str_ends_with", "array_is_list", "array_key_first", "array_key_last", "array_find",
    "array_any", "array_all", "json_validate", "mb_strcut", "opcache_get_status", "posix_kill", "pcntl_fork",
    "igbinary_serialize", "lz4_compress",
];

/// `get_defined_functions()`: the runtime's builtins as `internal`, the compiled program's functions
/// (a static table generated per crate) as `user`.
pub fn get_defined_functions(user_functions: &[&'static str]) -> Map<ArrayKey, Mixed> {
    let mut internal: Map<ArrayKey, Mixed> = Map::new();
    for n in BUILTIN_FUNCTION_NAMES {
        internal.push(Mixed::Str(Str::from_static(n)));
    }
    let mut user: Map<ArrayKey, Mixed> = Map::new();
    for n in user_functions {
        user.push(Mixed::Str(Str::from_static(n)));
    }
    let mut m: Map<ArrayKey, Mixed> = Map::new();
    m.insert(ArrayKey::from(Str::from_static("internal")), Mixed::Arr(internal));
    m.insert(ArrayKey::from(Str::from_static("user")), Mixed::Arr(user));
    m
}
pub fn opcache_get_status() -> Option<Mixed> {
    None
}
pub fn get_cfg_var(_n: &Str) -> Option<Str> {
    None
}
pub fn posix_kill(_pid: i64, _sig: i64) -> bool {
    false
}
pub fn posix_get_last_error() -> i64 {
    0
}
pub fn posix_strerror(_e: i64) -> Str {
    Str::from_static("Unknown error")
}
pub fn libxml_use_internal_errors(_b: bool) -> bool {
    false
}
pub fn libxml_clear_errors() {}
pub fn libxml_get_errors() -> List<Mixed> {
    List::new()
}
pub fn settype(_m: &Mixed, _t: &Str) -> bool {
    false
}
pub fn array_walk_recursive(_a: &Mixed, _cb: &Mixed) -> bool {
    true
}
pub fn parse_url(url: &Str, component: i64) -> Mixed {
    let s = url.to_string_lossy().into_owned();
    let mut m: Map<ArrayKey, Mixed> = Map::new();
    let (scheme, rest) = match s.split_once("://") {
        Some((a, b)) => (Some(a.to_string()), b.to_string()),
        None => (None, s.clone()),
    };
    if let Some(sc) = &scheme {
        m.insert(ArrayKey::from("scheme"), Mixed::Str(Str::from_str(sc)));
    }
    let (host_part, path) = if scheme.is_some() {
        match rest.find('/') {
            Some(i) => (Some(rest[..i].to_string()), rest[i..].to_string()),
            None => (Some(rest.clone()), String::new()),
        }
    } else {
        (None, rest.clone())
    };
    if let Some(h) = host_part {
        m.insert(ArrayKey::from("host"), Mixed::Str(Str::from_string(h)));
    }
    let (path, query) = match path.split_once('?') {
        Some((p, q)) => (p.to_string(), Some(q.to_string())),
        None => (path, None),
    };
    if !path.is_empty() {
        m.insert(ArrayKey::from("path"), Mixed::Str(Str::from_string(path)));
    }
    if let Some(q) = query {
        m.insert(ArrayKey::from("query"), Mixed::Str(Str::from_string(q)));
    }
    match component {
        -1 => Mixed::Arr(m),
        0 => m.get(&ArrayKey::from("scheme")).cloned().unwrap_or(Mixed::Null),
        1 => m.get(&ArrayKey::from("host")).cloned().unwrap_or(Mixed::Null),
        5 => m.get(&ArrayKey::from("path")).cloned().unwrap_or(Mixed::Null),
        6 => m.get(&ArrayKey::from("query")).cloned().unwrap_or(Mixed::Null),
        _ => Mixed::Null,
    }
}
pub fn filter_var(v: &Mixed, filter: i64, _options: Mixed) -> Mixed {
    let s = v.to_php_str();
    match filter {
        257 => match crate::conv::parse_numeric(s.as_bytes()) {
            Some(crate::conv::Num::Int(i)) => Mixed::Int(i),
            _ => Mixed::Bool(false),
        },
        259 => match crate::conv::parse_numeric(s.as_bytes()) {
            Some(n) => Mixed::Float(n.to_f64()),
            None => Mixed::Bool(false),
        },
        258 => match s.to_lowercase().as_bytes() {
            b"1" | b"true" | b"on" | b"yes" => Mixed::Bool(true),
            b"0" | b"false" | b"off" | b"no" | b"" => Mixed::Bool(false),
            _ => Mixed::Null,
        },
        273 => {
            if s.as_bytes().contains(&b':') { Mixed::Str(s) } else { Mixed::Bool(false) }
        }
        274 => {
            if s.as_bytes().contains(&b'@') { Mixed::Str(s) } else { Mixed::Bool(false) }
        }
        _ => Mixed::Str(s),
    }
}
pub fn getopt(_short: &Str, _long: List<Mixed>) -> Map<ArrayKey, Mixed> {
    Map::new()
}
/// `hrtime()`: (seconds, nanoseconds) of a monotonic clock.
pub fn hrtime_parts() -> (i64, i64) {
    thread_local! { static START: std::time::Instant = std::time::Instant::now(); }
    let d = START.with(|s| s.elapsed());
    (d.as_secs() as i64, d.subsec_nanos() as i64)
}

pub fn rt_function_is_builtin(name: &Str) -> bool {
    let lc = name.as_bytes().to_ascii_lowercase();
    let lc = if lc.first() == Some(&b'\\') { lc[1..].to_vec() } else { lc };
    builtin_function_exists(&lc) || BUILTIN_FUNCTION_NAMES.iter().any(|n| n.as_bytes() == lc.as_slice())
}

// ---------------------------------------------------------------- incremental hashing

/// `hash_init()` context: the algorithm and the data fed so far.
pub struct HashContext {
    pub algo: Str,
    pub data: crate::support::RwCell<Vec<u8>>,
}
impl crate::mixed::PhpObject for HashContext {
    fn class_name(&self) -> &'static str {
        "HashContext"
    }
    fn class_ancestors(&self) -> &'static [&'static str] {
        &["hashcontext"]
    }
    fn obj_id(&self) -> usize {
        self as *const _ as usize
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
pub fn hash_init(algo: &Str) -> Mixed {
    Mixed::Obj(std::sync::Arc::new(HashContext { algo: algo.clone(), data: crate::support::RwCell::new(Vec::new()) }))
}
pub fn hash_update(ctx: &Mixed, data: &Str) -> bool {
    if let Mixed::Obj(o) = ctx {
        if let Some(h) = o.as_any().downcast_ref::<HashContext>() {
            h.data.borrow_mut().extend_from_slice(data.as_bytes());
            return true;
        }
    }
    false
}
pub fn hash_final(ctx: &Mixed, _binary: bool) -> Str {
    if let Mixed::Obj(o) = ctx {
        if let Some(h) = o.as_any().downcast_ref::<HashContext>() {
            let data = Str::from_vec(h.data.borrow().clone());
            return crate::builtins::string::hash(&h.algo, &data).unwrap_or_default();
        }
    }
    Str::empty()
}

// ---------------------------------------------------------------- pack / unpack

/// One `pack()`/`unpack()` format code with its repeat count (None: `*`) and, for unpack, its key name.
fn parse_format(format: &[u8]) -> Vec<(u8, Option<usize>, Vec<u8>)> {
    let mut out = Vec::new();
    for part in format.split(|b| *b == b'/') {
        if part.is_empty() {
            continue;
        }
        let code = part[0];
        let mut i = 1;
        let mut count: Option<usize> = Some(1);
        if i < part.len() && part[i] == b'*' {
            count = None;
            i += 1;
        } else {
            let start = i;
            while i < part.len() && part[i].is_ascii_digit() {
                i += 1;
            }
            if i > start {
                count = Some(String::from_utf8_lossy(&part[start..i]).parse().unwrap_or(1));
            }
        }
        out.push((code, count, part[i..].to_vec()));
    }
    out
}

fn int_width(code: u8) -> Option<(usize, bool, bool)> {
    // (bytes, big endian, signed)
    match code {
        b'C' => Some((1, false, false)),
        b'c' => Some((1, false, true)),
        b'n' => Some((2, true, false)),
        b'v' | b'S' => Some((2, false, false)),
        b's' => Some((2, false, true)),
        b'N' => Some((4, true, false)),
        b'V' | b'L' => Some((4, false, false)),
        b'l' => Some((4, false, true)),
        b'J' => Some((8, true, false)),
        b'P' | b'Q' => Some((8, false, false)),
        b'q' => Some((8, false, true)),
        _ => None,
    }
}

pub fn pack(format: &Str, args: &Mixed) -> Str {
    let values: Vec<Mixed> = match args {
        Mixed::Arr(a) => a.iter().map(|(_, v)| v.clone()).collect(),
        other => vec![other.clone()],
    };
    let mut vi = 0;
    let mut out = Vec::new();
    for (code, count, _) in parse_format(format.as_bytes()) {
        match code {
            b'a' | b'A' | b'Z' => {
                let s = values.get(vi).map(|v| crate::traits::ToStr::to_php_str(v)).unwrap_or_default();
                vi += 1;
                let bytes = s.as_bytes();
                match count {
                    None => {
                        out.extend_from_slice(bytes);
                        if code == b'Z' {
                            out.push(0);
                        }
                    }
                    Some(n) => {
                        let take = bytes.len().min(n);
                        out.extend_from_slice(&bytes[..take]);
                        let pad = if code == b'A' { b' ' } else { 0 };
                        for _ in take..n {
                            out.push(pad);
                        }
                    }
                }
            }
            _ => {
                if let Some((width, big, _)) = int_width(code) {
                    let n = count.unwrap_or(values.len().saturating_sub(vi));
                    for _ in 0..n {
                        let v = values.get(vi).map(|v| crate::traits::ToInt::to_php_int(v)).unwrap_or(0) as u64;
                        vi += 1;
                        let bytes = if big { v.to_be_bytes() } else { v.to_le_bytes() };
                        if big {
                            out.extend_from_slice(&bytes[8 - width..]);
                        } else {
                            out.extend_from_slice(&bytes[..width]);
                        }
                    }
                }
            }
        }
    }
    Str::from_vec(out)
}

pub fn unpack(format: &Str, data: &Str, offset: i64) -> Map<ArrayKey, Mixed> {
    let bytes = data.as_bytes();
    let mut pos = offset.max(0) as usize;
    let mut out: Map<ArrayKey, Mixed> = Map::new();
    for (code, count, name) in parse_format(format.as_bytes()) {
        let key = |i: usize, single: bool| -> ArrayKey {
            if name.is_empty() {
                ArrayKey::Int(i as i64 + 1)
            } else if single {
                ArrayKey::from_str_val(Str::from_bytes(&name))
            } else {
                let mut k = name.clone();
                k.extend_from_slice((i + 1).to_string().as_bytes());
                ArrayKey::from_str_val(Str::from_bytes(&k))
            }
        };
        match code {
            b'a' | b'A' | b'Z' => {
                let n = count.unwrap_or(bytes.len().saturating_sub(pos));
                let end = (pos + n).min(bytes.len());
                let mut s = bytes[pos.min(end)..end].to_vec();
                if code == b'A' {
                    while s.last().map_or(false, |b| *b == b' ' || *b == 0) {
                        s.pop();
                    }
                }
                if code == b'Z' {
                    if let Some(p) = s.iter().position(|b| *b == 0) {
                        s.truncate(p);
                    }
                }
                out.insert(key(0, true), Mixed::Str(Str::from_vec(s)));
                pos = end;
            }
            _ => {
                if let Some((width, big, signed)) = int_width(code) {
                    let n = count.unwrap_or(bytes.len().saturating_sub(pos) / width.max(1));
                    for i in 0..n {
                        if pos + width > bytes.len() {
                            break;
                        }
                        let chunk = &bytes[pos..pos + width];
                        let mut buf = [0u8; 8];
                        let v: u64 = if big {
                            buf[8 - width..].copy_from_slice(chunk);
                            u64::from_be_bytes(buf)
                        } else {
                            buf[..width].copy_from_slice(chunk);
                            u64::from_le_bytes(buf)
                        };
                        let v = if signed && width < 8 {
                            let shift = 64 - width * 8;
                            ((v << shift) as i64) >> shift
                        } else {
                            v as i64
                        };
                        out.insert(key(i, n == 1 && count == Some(1)), Mixed::Int(v));
                        pos += width;
                    }
                }
            }
        }
    }
    out
}
pub fn serialize(m: &Mixed) -> Str {
    fn ser(m: &Mixed, out: &mut Vec<u8>) {
        match m {
            Mixed::Null => out.extend_from_slice(b"N;"),
            Mixed::Bool(b) => out.extend_from_slice(if *b { b"b:1;" } else { b"b:0;" }),
            Mixed::Int(i) => out.extend_from_slice(format!("i:{};", i).as_bytes()),
            Mixed::Float(f) => out.extend_from_slice(format!("d:{};", crate::conv::float_to_string_repr(*f)).as_bytes()),
            Mixed::Str(s) => {
                out.extend_from_slice(format!("s:{}:\"", s.len()).as_bytes());
                out.extend_from_slice(s);
                out.extend_from_slice(b"\";");
            }
            Mixed::Arr(a) => {
                out.extend_from_slice(format!("a:{}:{{", a.len()).as_bytes());
                for (k, v) in a.iter() {
                    match k {
                        ArrayKey::Int(i) => out.extend_from_slice(format!("i:{};", i).as_bytes()),
                        ArrayKey::Str(s) => {
                            out.extend_from_slice(format!("s:{}:\"", s.len()).as_bytes());
                            out.extend_from_slice(s);
                            out.extend_from_slice(b"\";");
                        }
                    }
                    ser(v, out);
                }
                out.push(b'}');
            }
            Mixed::Obj(o) => {
                let props = o.props();
                out.extend_from_slice(format!("O:{}:\"{}\":{}:{{", o.class_name().len(), o.class_name(), props.len()).as_bytes());
                for (k, v) in props {
                    out.extend_from_slice(format!("s:{}:\"", k.len()).as_bytes());
                    out.extend_from_slice(&k);
                    out.extend_from_slice(b"\";");
                    ser(&v, out);
                }
                out.push(b'}');
            }
            Mixed::Closure(_) => out.extend_from_slice(b"N;"),
        }
    }
    let mut out = Vec::new();
    ser(m, &mut out);
    Str::from_vec(out)
}
pub fn unserialize(_s: &Str) -> Mixed {
    Mixed::Bool(false)
}
pub fn get_parent_class_of(m: &Mixed) -> Option<Str> {
    match m {
        Mixed::Obj(o) => o.class_ancestors().get(1).map(|s| Str::from_str(s)),
        _ => None,
    }
}
pub fn get_object_vars(m: &Mixed) -> Map<ArrayKey, Mixed> {
    crate::support::object_to_array(m)
}
pub fn token_name(id: i64) -> Str {
    crate::consts::token_name(id)
}
pub fn token_get_all(code: &Str) -> List<Mixed> {
    let mut out = Vec::new();
    for (id, text, line, _pos) in crate::tokenizer::tokenize(code) {
        if id < 256 {
            out.push(Mixed::Str(text));
        } else {
            let mut t: Map<ArrayKey, Mixed> = Map::new();
            t.push(Mixed::Int(id));
            t.push(Mixed::Str(text));
            t.push(Mixed::Int(line));
            out.push(Mixed::Arr(t));
        }
    }
    List::from_vec(out)
}
/// Backing function for the PhpToken runtime stub.
pub fn __rt_tokenize(code: &Str) -> List<(i64, Str, i64, i64)> {
    List::from_vec(crate::tokenizer::tokenize(code))
}

pub fn simplexml_load_string(_s: &Str) -> Option<Mixed> {
    None
}
pub fn simplexml_load_file(_s: &Str) -> Option<Mixed> {
    None
}

/// Builtin functions callable by name (used by `call_user_func`, `array_map('strtolower', ...)`).
pub fn builtin_callable(lc: &[u8]) -> Option<DynCallable> {
    macro_rules! s1 {
        ($f:expr) => {
            Some(DynCallable::from_rt(1, |a| Ok(Mixed::Str($f(&a[0].to_php_str())))))
        };
    }
    match lc {
        b"strtolower" => s1!(crate::builtins::string::strtolower),
        b"strtoupper" => s1!(crate::builtins::string::strtoupper),
        b"ucfirst" => s1!(crate::builtins::string::ucfirst),
        b"lcfirst" => s1!(crate::builtins::string::lcfirst),
        b"trim" => s1!(|s: &Str| crate::builtins::string::trim(s, None)),
        b"strval" => s1!(|s: &Str| s.clone()),
        b"strlen" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Int(a[0].to_php_str().len() as i64)))),
        b"intval" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Int(a[0].to_php_int())))),
        b"is_string" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_string())))),
        b"is_int" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_int())))),
        b"is_array" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_array())))),
        b"is_null" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_null())))),
        b"is_numeric" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_numeric())))),
        b"is_object" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_object())))),
        b"is_bool" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_bool())))),
        b"is_float" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_float())))),
        b"is_scalar" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(a[0].is_scalar())))),
        b"strcmp" => Some(DynCallable::from_rt(2, |a| Ok(Mixed::Int(crate::builtins::string::strcmp(&a[0].to_php_str(), &a[1].to_php_str()))))),
        b"strcasecmp" => Some(DynCallable::from_rt(2, |a| Ok(Mixed::Int(crate::builtins::string::strcasecmp(&a[0].to_php_str(), &a[1].to_php_str()))))),
        b"strnatcmp" => Some(DynCallable::from_rt(2, |a| Ok(Mixed::Int(crate::builtins::string::strnatcmp(&a[0].to_php_str(), &a[1].to_php_str()))))),
        b"strnatcasecmp" => Some(DynCallable::from_rt(2, |a| Ok(Mixed::Int(crate::builtins::string::strnatcasecmp(&a[0].to_php_str(), &a[1].to_php_str()))))),
        b"ucwords" => s1!(crate::builtins::string::ucwords),
        b"ltrim" => s1!(|s: &Str| crate::builtins::string::ltrim(s, None)),
        b"rtrim" => s1!(|s: &Str| crate::builtins::string::rtrim(s, None)),
        b"md5" => s1!(crate::builtins::string::md5),
        b"floatval" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Float(a[0].to_php_float())))),
        b"boolval" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Bool(crate::traits::Truthy::truthy(&a[0]))))),
        b"count" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Int(a[0].php_count())))),
        b"array_values" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Arr(crate::support::mixed_to_array(a[0].clone()).renumbered())))),
        b"array_keys" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Arr(crate::support::mixed_to_array(a[0].clone()).keys_list().into_iter().enumerate().map(|(i, k)| (ArrayKey::Int(i as i64), Mixed::from(k))).collect())))),
        b"array_unique" => Some(DynCallable::from_rt(1, |a| Ok(Mixed::Arr(crate::builtins::array::array_unique_m(&crate::support::mixed_to_array(a[0].clone())))))),
        b"array_merge" => Some(DynCallable::from_rt(0, |a| {
            let parts: Vec<Map<ArrayKey, Mixed>> = a.into_iter().map(crate::support::mixed_to_array).collect();
            let refs: Vec<&Map<ArrayKey, Mixed>> = parts.iter().collect();
            Ok(Mixed::Arr(crate::builtins::array::array_merge_m(&refs)))
        })),
        _ => None,
    }
}

pub fn builtin_function_exists(lc: &[u8]) -> bool {
    builtin_callable(lc).is_some()
        || matches!(
            lc,
            b"preg_match" | b"array_map" | b"array_filter" | b"str_contains" | b"str_starts_with" | b"str_ends_with" | b"array_is_list" | b"array_key_first" | b"array_key_last" | b"array_find" | b"array_any" | b"array_all" | b"json_validate" | b"mb_strcut" | b"opcache_get_status" | b"posix_kill" | b"pcntl_fork" | b"igbinary_serialize" | b"lz4_compress"
        )
}

/// `defined()` on a runtime-provided constant name.
pub fn builtin_constant_defined(name: &Str) -> bool {
    get_defined_constants(false).get(&ArrayKey::from(name.clone())).is_some()
}
