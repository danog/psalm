//! Filesystem and stream functions.

use crate::containers::{new_resource, Resource, ResourceKind};
use crate::error::RtError;
use crate::list::List;
use crate::string::Str;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Arc as Rc;

fn path(s: &Str) -> std::path::PathBuf {
    std::path::PathBuf::from(std::ffi::OsStr::new(&*s.to_string_lossy()))
}

pub fn file_get_contents(p: &Str) -> Option<Str> {
    if p.as_bytes() == b"php://stdin" {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).ok()?;
        return Some(Str::from_vec(buf));
    }
    std::fs::read(path(p)).ok().map(Str::from_vec)
}

pub fn file_put_contents(p: &Str, data: &Str, flags: i64) -> Option<i64> {
    let res = if flags & 8 != 0 {
        std::fs::OpenOptions::new().append(true).create(true).open(path(p)).and_then(|mut f| f.write_all(data))
    } else {
        std::fs::write(path(p), data.as_bytes())
    };
    res.ok().map(|_| data.len() as i64)
}

pub fn file_exists(p: &Str) -> bool {
    path(p).exists()
}
pub fn is_dir(p: &Str) -> bool {
    path(p).is_dir()
}
pub fn is_file(p: &Str) -> bool {
    path(p).is_file()
}
pub fn is_link(p: &Str) -> bool {
    std::fs::symlink_metadata(path(p)).map(|m| m.file_type().is_symlink()).unwrap_or(false)
}
pub fn is_readable(p: &Str) -> bool {
    path(p).exists()
}
pub fn is_writable(p: &Str) -> bool {
    std::fs::metadata(path(p)).map(|m| !m.permissions().readonly()).unwrap_or(false)
}
pub fn realpath(p: &Str) -> Option<Str> {
    std::fs::canonicalize(path(p)).ok().map(|pb| Str::from_string(pb.to_string_lossy().into_owned()))
}
pub fn getcwd() -> Str {
    std::env::current_dir().map(|p| Str::from_string(p.to_string_lossy().into_owned())).unwrap_or_default()
}
pub fn chdir(p: &Str) -> bool {
    std::env::set_current_dir(path(p)).is_ok()
}
pub fn mkdir(p: &Str, _mode: i64, recursive: bool) -> bool {
    if recursive { std::fs::create_dir_all(path(p)).is_ok() } else { std::fs::create_dir(path(p)).is_ok() }
}
pub fn rmdir(p: &Str) -> bool {
    std::fs::remove_dir(path(p)).is_ok()
}
pub fn unlink(p: &Str) -> bool {
    std::fs::remove_file(path(p)).is_ok()
}
pub fn rename(a: &Str, b: &Str) -> bool {
    std::fs::rename(path(a), path(b)).is_ok()
}
pub fn copy(a: &Str, b: &Str) -> bool {
    std::fs::copy(path(a), path(b)).is_ok()
}
pub fn touch(p: &Str, mtime: Option<i64>) -> bool {
    let pb = path(p);
    let ok = if pb.exists() {
        std::fs::OpenOptions::new().append(true).open(&pb).is_ok()
    } else {
        std::fs::write(&pb, b"").is_ok()
    };
    match (ok, mtime) {
        (true, Some(t)) => {
            let when = std::time::UNIX_EPOCH + std::time::Duration::from_secs(t.max(0) as u64);
            match std::fs::File::options().write(true).open(&pb) {
                Ok(f) => f.set_modified(when).is_ok(),
                Err(_) => false,
            }
        }
        _ => ok,
    }
}
pub fn filemtime(p: &Str) -> Option<i64> {
    std::fs::metadata(path(p)).ok()?.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs() as i64)
}
pub fn filesize(p: &Str) -> Option<i64> {
    std::fs::metadata(path(p)).ok().map(|m| m.len() as i64)
}
pub fn tempnam(dir: &Str, prefix: &Str) -> Str {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let name = crate::sfmt!("{}/{}{:x}", dir, prefix, t);
    let _ = std::fs::write(path(&name), b"");
    name
}
pub fn sys_get_temp_dir() -> Str {
    Str::from_string(std::env::temp_dir().to_string_lossy().trim_end_matches('/').to_string())
}
pub fn readlink(p: &Str) -> Option<Str> {
    std::fs::read_link(path(p)).ok().map(|pb| Str::from_string(pb.to_string_lossy().into_owned()))
}
pub fn symlink(target: &Str, link: &Str) -> bool {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(path(target), path(link)).is_ok()
    }
    #[cfg(not(unix))]
    {
        false
    }
}
/// `sorting_order`: SCANDIR_SORT_ASCENDING (0), SCANDIR_SORT_DESCENDING (1), SCANDIR_SORT_NONE (2).
pub fn scandir(p: &Str, sorting_order: i64) -> List<Str> {
    let mut names: Vec<Str> = vec![Str::from_static("."), Str::from_static("..")];
    if let Ok(rd) = std::fs::read_dir(path(p)) {
        for e in rd.flatten() {
            names.push(Str::from_string(e.file_name().to_string_lossy().into_owned()));
        }
    }
    match sorting_order {
        2 => {}
        1 => names.sort_by(|a, b| b.cmp(a)),
        _ => names.sort(),
    }
    List::from_vec(names)
}
pub fn file_lines(p: &Str, flags: i64) -> List<Str> {
    let content = match file_get_contents(p) {
        Some(c) => c,
        None => return List::new(),
    };
    let ignore_nl = flags & 2 != 0;
    let skip_empty = flags & 4 != 0;
    let mut out = Vec::new();
    let b = content.as_bytes();
    let mut start = 0;
    for (i, &c) in b.iter().enumerate() {
        if c == b'\n' {
            let end = if ignore_nl { i } else { i + 1 };
            let line = &b[start..end];
            if !(skip_empty && line.is_empty()) {
                out.push(Str::from_bytes(line));
            }
            start = i + 1;
        }
    }
    if start < b.len() {
        out.push(Str::from_bytes(&b[start..]));
    }
    List::from_vec(out)
}

/// Minimal glob supporting `*`, `?`, `[...]` in path segments and `{a,b}` braces.
pub fn glob(pattern: &Str, flags: i64) -> List<Str> {
    let pat = pattern.to_string_lossy().into_owned();
    let mut pats = vec![pat.clone()];
    if flags & 1024 != 0 {
        pats = expand_braces(&pat);
    }
    let mut out: Vec<Str> = Vec::new();
    for p in pats {
        glob_one(&p, flags, &mut out);
    }
    if flags & 4 == 0 {
        out.sort();
    }
    List::from_vec(out)
}

fn expand_braces(p: &str) -> Vec<String> {
    if let Some(open) = p.find('{') {
        let mut depth = 0;
        let mut close = None;
        for (i, c) in p[open..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        close = Some(open + i);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(close) = close {
            let inner = &p[open + 1..close];
            let mut out = Vec::new();
            for alt in inner.split(',') {
                let s = format!("{}{}{}", &p[..open], alt, &p[close + 1..]);
                out.extend(expand_braces(&s));
            }
            return out;
        }
    }
    vec![p.to_string()]
}

fn glob_one(pattern: &str, flags: i64, out: &mut Vec<Str>) {
    let absolute = pattern.starts_with('/');
    let segments: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let mut current: Vec<std::path::PathBuf> = vec![if absolute { std::path::PathBuf::from("/") } else { std::path::PathBuf::from("") }];
    for (idx, seg) in segments.iter().enumerate() {
        let last = idx == segments.len() - 1;
        let mut next = Vec::new();
        for base in &current {
            if !has_wild(seg) {
                let p = if base.as_os_str().is_empty() { std::path::PathBuf::from(seg) } else { base.join(seg) };
                if p.exists() || !last {
                    if last && !p.exists() {
                        continue;
                    }
                    next.push(p);
                }
                continue;
            }
            let dir = if base.as_os_str().is_empty() { std::path::PathBuf::from(".") } else { base.clone() };
            if let Ok(rd) = std::fs::read_dir(&dir) {
                let mut entries: Vec<_> = rd.flatten().collect();
                entries.sort_by_key(|e| e.file_name());
                for e in entries {
                    let name = e.file_name().to_string_lossy().into_owned();
                    if name.starts_with('.') && !seg.starts_with('.') {
                        continue;
                    }
                    if wild_match(seg.as_bytes(), name.as_bytes()) {
                        let p = if base.as_os_str().is_empty() { std::path::PathBuf::from(&name) } else { base.join(&name) };
                        next.push(p);
                    }
                }
            }
        }
        current = next;
    }
    for p in current {
        if flags & 8192 != 0 && !p.is_dir() {
            continue;
        }
        let mut s = p.to_string_lossy().into_owned();
        if flags & 2 != 0 && p.is_dir() {
            s.push('/');
        }
        out.push(Str::from_string(s));
    }
}

fn has_wild(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

pub fn wild_match(pat: &[u8], s: &[u8]) -> bool {
    if pat.is_empty() {
        return s.is_empty();
    }
    match pat[0] {
        b'*' => (0..=s.len()).any(|i| wild_match(&pat[1..], &s[i..])),
        b'?' => !s.is_empty() && wild_match(&pat[1..], &s[1..]),
        b'[' => {
            if s.is_empty() {
                return false;
            }
            if let Some(end) = pat.iter().position(|&c| c == b']') {
                let set = &pat[1..end];
                let (neg, set) = if set.first() == Some(&b'!') || set.first() == Some(&b'^') { (true, &set[1..]) } else { (false, set) };
                let mut hit = false;
                let mut i = 0;
                while i < set.len() {
                    if i + 2 < set.len() && set[i + 1] == b'-' {
                        if s[0] >= set[i] && s[0] <= set[i + 2] {
                            hit = true;
                        }
                        i += 3;
                    } else {
                        if set[i] == s[0] {
                            hit = true;
                        }
                        i += 1;
                    }
                }
                hit != neg && wild_match(&pat[end + 1..], &s[1..])
            } else {
                !s.is_empty() && s[0] == b'[' && wild_match(&pat[1..], &s[1..])
            }
        }
        c => !s.is_empty() && s[0] == c && wild_match(&pat[1..], &s[1..]),
    }
}

// ---------------------------------------------------------------- streams

pub fn fopen(p: &Str, mode: &Str) -> Option<Rc<Resource>> {
    match p.as_bytes() {
        b"php://stdin" => return Some(crate::containers::stdin_res()),
        b"php://stdout" | b"php://output" => return Some(crate::containers::stdout_res()),
        b"php://stderr" => return Some(crate::containers::stderr_res()),
        b"php://memory" | b"php://temp" => return Some(new_resource(ResourceKind::Memory(crate::support::RwCell::new(Vec::new()), std::sync::atomic::AtomicUsize::new(0)))),
        _ => {}
    }
    let m = mode.as_bytes();
    let mut o = std::fs::OpenOptions::new();
    let plus = m.contains(&b'+');
    match m.first() {
        Some(b'r') => {
            o.read(true).write(plus);
        }
        Some(b'w') => {
            o.write(true).create(true).truncate(true).read(plus);
        }
        Some(b'a') => {
            o.append(true).create(true).read(plus);
        }
        Some(b'x') => {
            o.write(true).create_new(true).read(plus);
        }
        Some(b'c') => {
            o.write(true).create(true).read(plus);
        }
        _ => return None,
    }
    let f = o.open(path(p)).ok()?;
    Some(new_resource(ResourceKind::File(crate::support::RwCell::new(f), p.clone())))
}

pub fn fwrite(r: &Rc<Resource>, data: &Str) -> Option<i64> {
    let kind = r.kind.borrow();
    let ok = match &*kind {
        ResourceKind::Stdout => {
            crate::output::echo(data);
            true
        }
        ResourceKind::Stderr => {
            crate::output::eprint(data);
            true
        }
        ResourceKind::File(f, _) => f.borrow_mut().write_all(data).is_ok(),
        ResourceKind::Memory(buf, pos) => {
            let mut b = buf.borrow_mut();
            let p = pos.load(std::sync::atomic::Ordering::Relaxed);
            if p >= b.len() {
                b.extend_from_slice(data);
            } else {
                let end = (p + data.len()).min(b.len());
                b[p..end].copy_from_slice(&data[..end - p]);
                b.extend_from_slice(&data[end - p..]);
            }
            pos.store(p + data.len(), std::sync::atomic::Ordering::Relaxed);
            true
        }
        _ => false,
    };
    if ok { Some(data.len() as i64) } else { None }
}

pub fn fclose(r: &Rc<Resource>) -> bool {
    let mut kind = r.kind.borrow_mut();
    match &*kind {
        ResourceKind::Stdin | ResourceKind::Stdout | ResourceKind::Stderr => true,
        _ => {
            *kind = ResourceKind::Closed;
            true
        }
    }
}
pub fn fflush(r: &Rc<Resource>) -> bool {
    if let ResourceKind::File(f, _) = &*r.kind.borrow() {
        return f.borrow_mut().flush().is_ok();
    }
    true
}
pub fn flock(_r: &Rc<Resource>, _op: i64) -> bool {
    true
}
pub fn fgets(r: &Rc<Resource>) -> Option<Str> {
    let kind = r.kind.borrow();
    match &*kind {
        ResourceKind::Stdin => {
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(0) => None,
                Ok(_) => Some(Str::from_string(line)),
                Err(_) => None,
            }
        }
        ResourceKind::File(f, _) => {
            let mut f = f.borrow_mut();
            let mut out = Vec::new();
            let mut byte = [0u8; 1];
            loop {
                match f.read(&mut byte) {
                    Ok(0) => break,
                    Ok(_) => {
                        out.push(byte[0]);
                        if byte[0] == b'\n' {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            if out.is_empty() { None } else { Some(Str::from_vec(out)) }
        }
        ResourceKind::Memory(buf, pos) => {
            let b = buf.borrow();
            let p = pos.load(std::sync::atomic::Ordering::Relaxed);
            if p >= b.len() {
                return None;
            }
            let end = b[p..].iter().position(|&c| c == b'\n').map(|i| p + i + 1).unwrap_or(b.len());
            pos.store(end, std::sync::atomic::Ordering::Relaxed);
            Some(Str::from_bytes(&b[p..end]))
        }
        _ => None,
    }
}
pub fn fread(r: &Rc<Resource>, len: i64) -> Option<Str> {
    let kind = r.kind.borrow();
    let len = len.max(0) as usize;
    match &*kind {
        ResourceKind::File(f, _) => {
            let mut buf = vec![0u8; len];
            let n = f.borrow_mut().read(&mut buf).ok()?;
            buf.truncate(n);
            Some(Str::from_vec(buf))
        }
        ResourceKind::Memory(buf, pos) => {
            let b = buf.borrow();
            let p = pos.load(std::sync::atomic::Ordering::Relaxed).min(b.len());
            let end = (p + len).min(b.len());
            pos.store(end, std::sync::atomic::Ordering::Relaxed);
            Some(Str::from_bytes(&b[p..end]))
        }
        ResourceKind::Stdin => {
            let mut buf = vec![0u8; len];
            let n = std::io::stdin().read(&mut buf).ok()?;
            buf.truncate(n);
            Some(Str::from_vec(buf))
        }
        _ => None,
    }
}
pub fn feof(r: &Rc<Resource>) -> bool {
    let kind = r.kind.borrow();
    match &*kind {
        ResourceKind::File(f, _) => {
            let mut f = f.borrow_mut();
            let pos = f.stream_position().unwrap_or(0);
            let len = f.metadata().map(|m| m.len()).unwrap_or(0);
            pos >= len
        }
        ResourceKind::Memory(buf, pos) => pos.load(std::sync::atomic::Ordering::Relaxed) >= buf.borrow().len(),
        _ => true,
    }
}
pub fn ftruncate(r: &Rc<Resource>, size: i64) -> bool {
    let kind = r.kind.borrow();
    match &*kind {
        ResourceKind::File(f, _) => f.borrow_mut().set_len(size.max(0) as u64).is_ok(),
        ResourceKind::Memory(buf, _) => {
            buf.borrow_mut().truncate(size.max(0) as usize);
            true
        }
        _ => false,
    }
}
pub fn rewind(r: &Rc<Resource>) -> bool {
    let kind = r.kind.borrow();
    match &*kind {
        ResourceKind::File(f, _) => f.borrow_mut().seek(SeekFrom::Start(0)).is_ok(),
        ResourceKind::Memory(_, pos) => {
            pos.store(0, std::sync::atomic::Ordering::Relaxed);
            true
        }
        _ => false,
    }
}
pub fn stream_get_contents(r: &Rc<Resource>) -> Option<Str> {
    let kind = r.kind.borrow();
    match &*kind {
        ResourceKind::File(f, _) => {
            let mut buf = Vec::new();
            f.borrow_mut().read_to_end(&mut buf).ok()?;
            Some(Str::from_vec(buf))
        }
        ResourceKind::Memory(buf, pos) => {
            let b = buf.borrow();
            let p = pos.load(std::sync::atomic::Ordering::Relaxed).min(b.len());
            pos.store(b.len(), std::sync::atomic::Ordering::Relaxed);
            Some(Str::from_bytes(&b[p..]))
        }
        ResourceKind::Stdin => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf).ok()?;
            Some(Str::from_vec(buf))
        }
        _ => None,
    }
}
/// `stream_get_meta_data`: the metadata of a stream (the runtime's streams are blocking, unread, plain files/stdio).
pub struct StreamMeta {
    pub timed_out: bool,
    pub blocked: bool,
    pub eof: bool,
    pub stream_type: Str,
    pub mode: Str,
    pub unread_bytes: i64,
    pub seekable: bool,
    pub uri: Str,
}

pub fn stream_meta(_r: &Rc<Resource>) -> StreamMeta {
    StreamMeta {
        timed_out: false,
        blocked: true,
        eof: false,
        stream_type: Str::from_static("STDIO"),
        mode: Str::from_static("r"),
        unread_bytes: 0,
        seekable: false,
        uri: Str::from_static("php://stdin"),
    }
}

pub fn stream_set_blocking(_r: &Rc<Resource>, _b: bool) -> bool {
    true
}
pub fn is_resource(m: &crate::mixed::Mixed) -> bool {
    let _ = m;
    false
}
pub fn fprintf(r: &Rc<Resource>, format: &Str, args: &[crate::builtins::string::FmtArg]) -> i64 {
    match crate::builtins::string::sprintf(format, args) {
        Ok(s) => fwrite(r, &s).unwrap_or(0),
        Err(_) => 0,
    }
}
pub fn printf(format: &Str, args: &[crate::builtins::string::FmtArg]) -> i64 {
    match crate::builtins::string::sprintf(format, args) {
        Ok(s) => {
            crate::output::echo(&s);
            s.len() as i64
        }
        Err(_) => 0,
    }
}
pub fn readline(prompt: Option<&Str>) -> Option<Str> {
    if let Some(p) = prompt {
        crate::output::echo(p);
    }
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(Str::from_string(line.trim_end_matches('\n').to_string())),
        Err(_) => None,
    }
}
pub fn exec(_cmd: &Str) -> Option<Str> {
    None
}
pub fn passthru(_cmd: &Str) {}
pub fn gzdeflate(_s: &Str) -> Option<Str> {
    None
}
pub fn gzinflate(_s: &Str) -> Option<Str> {
    None
}
pub fn lz4_compress(_s: &Str) -> Option<Str> {
    None
}
pub fn lz4_uncompress(_s: &Str) -> Option<Str> {
    None
}
pub fn clearstatcache(_clear_realpath_cache: bool, _filename: &Str) {}
