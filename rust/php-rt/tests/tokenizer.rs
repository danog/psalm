//! Differential test: compare the Rust tokenizer against PHP's token_get_all() output
//! recorded in build/tokens.jsonl (produced by bin/transpile/dump-tokens.php).

use php_rt::prelude::*;

#[test]
fn tokenizer_matches_php() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../build/tokens.jsonl");
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("no reference file at {path}, skipping");
            return;
        }
    };
    let mut cases = 0;
    let mut failures = 0;
    for line in data.split(|&b| b == b'\n') {
        if line.is_empty() {
            continue;
        }
        let v = php_rt::json_decode(&Str::from_bytes(line), true, 512, 0).unwrap();
        let m = match v.as_arr() {
            Some(m) => m,
            None => {
                eprintln!("could not decode reference line ({} bytes): {}", line.len(), String::from_utf8_lossy(&line[..line.len().min(120)]));
                continue;
            }
        };
        let file = m.get(&ArrayKey::from("file")).unwrap().to_php_str();
        let code = m.get(&ArrayKey::from("code")).unwrap().to_php_str();
        let expected: Vec<(Str, Str, i64)> = m
            .get(&ArrayKey::from("tokens"))
            .unwrap()
            .as_arr()
            .unwrap()
            .values()
            .map(|t| {
                let t = t.as_arr().unwrap();
                (t.get(&ArrayKey::Int(0)).unwrap().to_php_str(), t.get(&ArrayKey::Int(1)).unwrap().to_php_str(), t.get(&ArrayKey::Int(2)).unwrap().to_php_int())
            })
            .collect();
        let actual: Vec<(Str, Str, i64)> = php_rt::tokenizer::tokenize(&code)
            .into_iter()
            .map(|(id, text, line, _)| (if id < 256 { Str::from_vec(vec![id as u8]) } else { php_rt::consts::token_name(id) }, text, line))
            .collect();
        cases += 1;
        let mut ok = actual.len() == expected.len();
        let mut first_bad = None;
        for (i, (a, e)) in actual.iter().zip(expected.iter()).enumerate() {
            let line_ok = e.2 == -1 || a.2 == e.2;
            let name_ok = a.0 == e.0 || (e.2 == -1 && a.1 == e.1);
            if !name_ok || a.1 != e.1 || !line_ok {
                ok = false;
                first_bad = Some(i);
                break;
            }
        }
        if !ok {
            failures += 1;
            if failures <= 15 {
                let i = first_bad.unwrap_or(actual.len().min(expected.len()));
                eprintln!("MISMATCH in {file} at token #{i} (php={} rust={})", expected.len(), actual.len());
                for j in i.saturating_sub(2)..(i + 3).min(expected.len().max(actual.len())) {
                    let e = expected.get(j).map(|t| format!("{}:{:?}@{}", t.0, t.1, t.2)).unwrap_or_default();
                    let a = actual.get(j).map(|t| format!("{}:{:?}@{}", t.0, t.1, t.2)).unwrap_or_default();
                    eprintln!("   #{j}: php {e}   rust {a}");
                }
            }
        }
    }
    eprintln!("{cases} cases, {failures} mismatching");
    assert_eq!(failures, 0);
}
