//! Math functions.

use crate::conv::Num;
use std::cell::Cell;

pub fn abs_i(i: i64) -> i64 {
    i.wrapping_abs()
}
pub fn abs_f(f: f64) -> f64 {
    f.abs()
}
pub fn ceil(f: f64) -> f64 {
    f.ceil()
}
pub fn floor(f: f64) -> f64 {
    f.floor()
}
/// PHP round(): half away from zero with pre-rounding fuzz.
pub fn round(value: f64, places: i64) -> f64 {
    if !value.is_finite() || value == 0.0 {
        return value;
    }
    let places = places.clamp(-308, 308) as i32;
    let f1 = 10f64.powi(places.abs());
    let tmp = if places >= 0 { value * f1 } else { value / f1 };
    if tmp.abs() >= 1e15 {
        return value;
    }
    // round half away from zero, with a small correction for representation error
    let r = tmp.round();
    let diff = (tmp - tmp.trunc()).abs();
    let rounded = if (diff - 0.5).abs() < 1e-9 * tmp.abs().max(1.0) {
        // exactly half after fuzz: away from zero
        if tmp >= 0.0 { tmp.trunc() + 1.0 } else { tmp.trunc() - 1.0 }
    } else {
        r
    };
    let res = if places >= 0 { rounded / f1 } else { rounded * f1 };
    // use string round-trip to kill representation noise like 1.0049999999
    let s = format!("{:.*}", places.max(0) as usize, res);
    s.parse::<f64>().unwrap_or(res)
}
pub fn round_i(value: i64, places: i64) -> f64 {
    round(value as f64, places)
}
pub fn sqrt(f: f64) -> f64 {
    f.sqrt()
}
pub fn is_nan(f: f64) -> bool {
    f.is_nan()
}
pub fn is_finite(f: f64) -> bool {
    f.is_finite()
}
pub fn is_infinite(f: f64) -> bool {
    f.is_infinite()
}
pub fn intval_num(n: Num) -> i64 {
    n.to_i64()
}
pub fn log(f: f64) -> f64 {
    f.ln()
}
pub fn log10(f: f64) -> f64 {
    f.log10()
}
pub fn exp(f: f64) -> f64 {
    f.exp()
}

thread_local! {
    static RNG: Cell<u64> = Cell::new(0x9E3779B97F4A7C15 ^ (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(1)));
}
fn next_u64() -> u64 {
    RNG.with(|r| {
        let mut x = r.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        r.set(x);
        x
    })
}
pub fn mt_srand(seed: i64) {
    RNG.with(|r| r.set((seed as u64) | 1));
}
pub fn mt_rand(min: i64, max: i64) -> i64 {
    if max <= min {
        return min;
    }
    let range = (max - min) as u64 + 1;
    min + (next_u64() % range) as i64
}
pub fn mt_rand0() -> i64 {
    (next_u64() >> 33) as i64
}
pub fn rand(min: i64, max: i64) -> i64 {
    mt_rand(min, max)
}
pub fn random_int(min: i64, max: i64) -> i64 {
    mt_rand(min, max)
}
pub fn mt_getrandmax() -> i64 {
    2147483647
}
pub fn microtime(_as_float: bool) -> f64 {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    t.as_secs_f64()
}
pub fn hrtime_ns() -> i64 {
    thread_local! { static START: std::time::Instant = std::time::Instant::now(); }
    START.with(|s| s.elapsed().as_nanos() as i64)
}
pub fn time() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
}
pub fn fdiv(a: f64, b: f64) -> f64 {
    a / b
}
pub fn dec_to_num(f: f64) -> Num {
    if f.fract() == 0.0 && f.abs() < 9.2e18 { Num::Int(f as i64) } else { Num::Float(f) }
}
