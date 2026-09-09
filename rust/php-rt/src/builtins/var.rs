//! Variable handling: type predicates, dumps.

use crate::conv;
use crate::key::ArrayKey;
use crate::map::Map;
use crate::mixed::Mixed;
use crate::string::Str;

pub fn gettype(m: &Mixed) -> Str {
    Str::from_static(m.type_name())
}
pub fn get_debug_type(m: &Mixed) -> Str {
    m.debug_type()
}

fn export_str(s: &Str, out: &mut Vec<u8>) {
    out.push(b'\'');
    for &c in s.as_bytes() {
        match c {
            b'\'' => out.extend_from_slice(b"\\'"),
            b'\\' => out.extend_from_slice(b"\\\\"),
            0 => out.extend_from_slice(b"' . \"\\0\" . '"),
            _ => out.push(c),
        }
    }
    out.push(b'\'');
}

fn var_export_inner(m: &Mixed, indent: usize, out: &mut Vec<u8>) {
    let pad = |n: usize| " ".repeat(n);
    match m {
        Mixed::Null => out.extend_from_slice(b"NULL"),
        Mixed::Bool(b) => out.extend_from_slice(if *b { b"true" } else { b"false" }),
        Mixed::Int(i) => out.extend_from_slice(i.to_string().as_bytes()),
        Mixed::Float(f) => out.extend_from_slice(conv::float_to_export(*f).as_bytes()),
        Mixed::Str(s) => export_str(s, out),
        Mixed::Arr(a) => {
            out.extend_from_slice(b"array (\n");
            for (k, v) in a.iter() {
                out.extend_from_slice(pad(indent + 2).as_bytes());
                match k {
                    ArrayKey::Int(i) => out.extend_from_slice(i.to_string().as_bytes()),
                    ArrayKey::Str(s) => export_str(s, out),
                }
                out.extend_from_slice(b" => ");
                if matches!(v, Mixed::Arr(_) | Mixed::Obj(_)) {
                    out.extend_from_slice(b"\n");
                    out.extend_from_slice(pad(indent + 2).as_bytes());
                }
                var_export_inner(v, indent + 2, out);
                out.extend_from_slice(b",\n");
            }
            out.extend_from_slice(pad(indent).as_bytes());
            out.push(b')');
        }
        Mixed::Obj(o) => {
            out.extend_from_slice(b"\\");
            out.extend_from_slice(o.class_name().as_bytes());
            out.extend_from_slice(b"::__set_state(array(\n");
            for (k, v) in o.props() {
                out.extend_from_slice(pad(indent + 3).as_bytes());
                export_str(&k, out);
                out.extend_from_slice(b" => ");
                if matches!(v, Mixed::Arr(_) | Mixed::Obj(_)) {
                    out.extend_from_slice(b"\n");
                    out.extend_from_slice(pad(indent + 2).as_bytes());
                }
                var_export_inner(&v, indent + 2, out);
                out.extend_from_slice(b",\n");
            }
            out.extend_from_slice(pad(indent).as_bytes());
            out.extend_from_slice(b"))");
        }
        Mixed::Closure(_) => out.extend_from_slice(b"\\Closure::__set_state(array(\n))"),
    }
}
pub fn var_export(m: &Mixed, ret: bool) -> Str {
    let mut out = Vec::new();
    var_export_inner(m, 0, &mut out);
    if !ret {
        crate::output::echo(&out);
        return Str::empty();
    }
    Str::from_vec(out)
}

fn print_r_inner(m: &Mixed, indent: usize, out: &mut Vec<u8>) {
    match m {
        Mixed::Arr(a) => {
            out.extend_from_slice(b"Array\n");
            out.extend_from_slice(" ".repeat(indent).as_bytes());
            out.extend_from_slice(b"(\n");
            for (k, v) in a.iter() {
                out.extend_from_slice(" ".repeat(indent + 4).as_bytes());
                out.extend_from_slice(format!("[{}] => ", k).as_bytes());
                print_r_inner(v, indent + 8, out);
                out.push(b'\n');
            }
            out.extend_from_slice(" ".repeat(indent).as_bytes());
            out.extend_from_slice(b")\n");
        }
        Mixed::Obj(o) => {
            out.extend_from_slice(o.class_name().as_bytes());
            out.extend_from_slice(b" Object\n");
            out.extend_from_slice(" ".repeat(indent).as_bytes());
            out.extend_from_slice(b"(\n");
            for (k, v) in o.props() {
                out.extend_from_slice(" ".repeat(indent + 4).as_bytes());
                out.extend_from_slice(format!("[{}] => ", k).as_bytes());
                print_r_inner(&v, indent + 8, out);
                out.push(b'\n');
            }
            out.extend_from_slice(" ".repeat(indent).as_bytes());
            out.extend_from_slice(b")\n");
        }
        other => out.extend_from_slice(crate::traits::ToStr::to_php_str(other).as_bytes()),
    }
}
pub fn print_r(m: &Mixed) -> Str {
    let mut out = Vec::new();
    print_r_inner(m, 0, &mut out);
    Str::from_vec(out)
}

fn var_dump_inner(m: &Mixed, indent: usize, out: &mut Vec<u8>) {
    let pad = " ".repeat(indent);
    match m {
        Mixed::Null => out.extend_from_slice(format!("{}NULL\n", pad).as_bytes()),
        Mixed::Bool(b) => out.extend_from_slice(format!("{}bool({})\n", pad, b).as_bytes()),
        Mixed::Int(i) => out.extend_from_slice(format!("{}int({})\n", pad, i).as_bytes()),
        Mixed::Float(f) => out.extend_from_slice(format!("{}float({})\n", pad, conv::float_to_string_repr(*f)).as_bytes()),
        Mixed::Str(s) => {
            out.extend_from_slice(format!("{}string({}) \"", pad, s.len()).as_bytes());
            out.extend_from_slice(s);
            out.extend_from_slice(b"\"\n");
        }
        Mixed::Arr(a) => {
            out.extend_from_slice(format!("{}array({}) {{\n", pad, a.len()).as_bytes());
            for (k, v) in a.iter() {
                match k {
                    ArrayKey::Int(i) => out.extend_from_slice(format!("{}  [{}]=>\n", pad, i).as_bytes()),
                    ArrayKey::Str(s) => out.extend_from_slice(format!("{}  [\"{}\"]=>\n", pad, s).as_bytes()),
                }
                var_dump_inner(v, indent + 2, out);
            }
            out.extend_from_slice(format!("{}}}\n", pad).as_bytes());
        }
        Mixed::Obj(o) => {
            let props = o.props();
            out.extend_from_slice(format!("{}object({})#{} ({}) {{\n", pad, o.class_name(), o.obj_id(), props.len()).as_bytes());
            for (k, v) in props {
                out.extend_from_slice(format!("{}  [\"{}\"]=>\n", pad, k).as_bytes());
                var_dump_inner(&v, indent + 2, out);
            }
            out.extend_from_slice(format!("{}}}\n", pad).as_bytes());
        }
        Mixed::Closure(_) => out.extend_from_slice(format!("{}object(Closure)#0 (0) {{\n{}}}\n", pad, pad).as_bytes()),
    }
}
pub fn var_dump(m: &Mixed) -> Str {
    let mut out = Vec::new();
    var_dump_inner(m, 0, &mut out);
    Str::from_vec(out)
}
pub fn intval(s: &Str, base: i64) -> i64 {
    if base == 10 {
        return conv::str_to_int(s);
    }
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() && conv::is_ws(b[i]) {
        i += 1;
    }
    let neg = i < b.len() && b[i] == b'-';
    if i < b.len() && (b[i] == b'-' || b[i] == b'+') {
        i += 1;
    }
    let mut base = base as u32;
    if base == 16 && b[i..].starts_with(b"0x") || base == 16 && b[i..].starts_with(b"0X") {
        i += 2;
    } else if base == 8 && b[i..].starts_with(b"0o") {
        i += 2;
    } else if base == 2 && b[i..].starts_with(b"0b") {
        i += 2;
    } else if base == 0 {
        if b[i..].starts_with(b"0x") || b[i..].starts_with(b"0X") {
            base = 16;
            i += 2;
        } else if b[i..].starts_with(b"0b") {
            base = 2;
            i += 2;
        } else if b[i..].starts_with(b"0") && b.len() > i + 1 {
            base = 8;
            i += 1;
        } else {
            base = 10;
        }
    }
    let mut v: i64 = 0;
    while i < b.len() {
        match (b[i] as char).to_digit(base) {
            Some(d) => v = v.saturating_mul(base as i64).saturating_add(d as i64),
            None => break,
        }
        i += 1;
    }
    if neg { -v } else { v }
}
pub fn boolval<T: crate::traits::Truthy>(v: &T) -> bool {
    v.truthy()
}
pub fn is_numeric_mixed(m: &Mixed) -> bool {
    m.is_numeric()
}
pub fn empty_map(m: &Map<ArrayKey, Mixed>) -> bool {
    m.is_empty()
}
