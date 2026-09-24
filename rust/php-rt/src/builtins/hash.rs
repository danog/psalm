//! Hash functions (md5, sha1, sha256, crc32, fnv) implemented natively.

pub struct Digest(pub [u8; 16]);
impl std::fmt::LowerHex for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

pub fn md5_digest(input: &[u8]) -> Digest {
    let s: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11,
        16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let k: Vec<u32> = (0..64).map(|i| ((i as f64 + 1.0).sin().abs() * 4294967296.0) as u32).collect();
    let (mut a0, mut b0, mut c0, mut d0) = (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);
    let mut msg = input.to_vec();
    let bit_len = (input.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());
    for chunk in msg.chunks(64) {
        let m: Vec<u32> = (0..16).map(|i| u32::from_le_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]])).collect();
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        for i in 0..64 {
            let (f, g) = match i / 16 {
                0 => ((b & c) | (!b & d), i),
                1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
                2 => (b ^ c ^ d, (3 * i + 5) % 16),
                _ => (c ^ (b | !d), (7 * i) % 16),
            };
            let f2 = f.wrapping_add(a).wrapping_add(k[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f2.rotate_left(s[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut out = [0u8; 16];
    out[..4].copy_from_slice(&a0.to_le_bytes());
    out[4..8].copy_from_slice(&b0.to_le_bytes());
    out[8..12].copy_from_slice(&c0.to_le_bytes());
    out[12..].copy_from_slice(&d0.to_le_bytes());
    Digest(out)
}

pub fn sha1_hex(input: &[u8]) -> String {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let mut msg = input.to_vec();
    let bit_len = (input.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = match i / 20 {
                0 => ((b & c) | (!b & d), 0x5A827999),
                1 => (b ^ c ^ d, 0x6ED9EBA1),
                2 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let t = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = t;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    h.iter().map(|x| format!("{:08x}", x)).collect()
}

pub fn sha256_hex(input: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut msg = input.to_vec();
    let bit_len = (input.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for i in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for i in 0..8 {
            h[i] = h[i].wrapping_add(v[i]);
        }
    }
    h.iter().map(|x| format!("{:08x}", x)).collect()
}

pub fn crc32(input: &[u8]) -> u32 {
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut c = i;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xEDB88320 ^ (c >> 1) } else { c >> 1 };
        }
        table[i as usize] = c;
    }
    let mut crc = 0xFFFFFFFFu32;
    for &b in input {
        crc = table[((crc ^ b as u32) & 0xff) as usize] ^ (crc >> 8);
    }
    crc ^ 0xFFFFFFFF
}

pub fn fnv64(input: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in input {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

// ---- XXH3 (64-bit, seed 0), ported from php-src ext/hash/xxhash/xxhash.h: PHP's hash('xxh3') ----

const XXH3_SECRET: [u8; 192] = [0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c, 0xde, 0xd4, 0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f, 0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21, 0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c, 0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3, 0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8, 0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d, 0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64, 0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb, 0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e, 0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce, 0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e];
const XXH_PRIME32_1: u64 = 0x9E3779B1;
const XXH_PRIME32_2: u64 = 0x85EBCA77;
const XXH_PRIME32_3: u64 = 0xC2B2AE3D;
const XXH_PRIME64_1: u64 = 0x9E3779B185EBCA87;
const XXH_PRIME64_2: u64 = 0xC2B2AE3D27D4EB4F;
const XXH_PRIME64_3: u64 = 0x165667B19E3779F9;
const XXH_PRIME64_4: u64 = 0x85EBCA77C2B2AE63;
const XXH_PRIME64_5: u64 = 0x27D4EB2F165667C5;
const XXH_PRIME_MX1: u64 = 0x165667919E3779F9;
const XXH_PRIME_MX2: u64 = 0x9FB21C651E98DF25;

#[inline(always)]
fn rd32(b: &[u8], i: usize) -> u64 {
    // one bounds check and one load (eight indexed byte reads were 48 instructions per call)
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap()) as u64
}
#[inline(always)]
fn rd64(b: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(b[i..i + 8].try_into().unwrap())
}
#[inline]
fn mul128_fold64(a: u64, b: u64) -> u64 {
    let p = (a as u128) * (b as u128);
    (p as u64) ^ ((p >> 64) as u64)
}
#[inline]
fn xxh64_avalanche(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(XXH_PRIME64_2);
    h ^= h >> 29;
    h = h.wrapping_mul(XXH_PRIME64_3);
    h ^ (h >> 32)
}
#[inline]
fn xxh3_avalanche(mut h: u64) -> u64 {
    h ^= h >> 37;
    h = h.wrapping_mul(XXH_PRIME_MX1);
    h ^ (h >> 32)
}
#[inline]
fn xxh3_rrmxmx(mut h: u64, len: u64) -> u64 {
    h ^= h.rotate_left(49) ^ h.rotate_left(24);
    h = h.wrapping_mul(XXH_PRIME_MX2);
    h ^= (h >> 35).wrapping_add(len);
    h = h.wrapping_mul(XXH_PRIME_MX2);
    h ^ (h >> 28)
}
#[inline]
fn xxh3_mix16b(input: &[u8], i: usize, secret: &[u8], j: usize) -> u64 {
    mul128_fold64(rd64(input, i) ^ rd64(secret, j), rd64(input, i + 8) ^ rd64(secret, j + 8))
}
fn xxh3_accumulate_512(acc: &mut [u64; 8], input: &[u8], i: usize, secret: &[u8], j: usize) {
    for lane in 0..8 {
        let data_val = rd64(input, i + 8 * lane);
        let data_key = data_val ^ rd64(secret, j + 8 * lane);
        acc[lane ^ 1] = acc[lane ^ 1].wrapping_add(data_val);
        acc[lane] = acc[lane].wrapping_add((data_key & 0xFFFF_FFFF).wrapping_mul(data_key >> 32));
    }
}
fn xxh3_scramble(acc: &mut [u64; 8], secret: &[u8], j: usize) {
    for lane in 0..8 {
        let key = rd64(secret, j + 8 * lane);
        let mut a = acc[lane];
        a ^= a >> 47;
        a ^= key;
        acc[lane] = a.wrapping_mul(XXH_PRIME32_1);
    }
}

/// `XXH3_64bits` with the default secret and seed 0: PHP's `hash('xxh3', $data)` as an integer.
pub fn xxh3_64(input: &[u8]) -> u64 {
    let s = &XXH3_SECRET;
    let len = input.len();
    if len <= 16 {
        if len > 8 {
            let bitflip1 = rd64(s, 24) ^ rd64(s, 32);
            let bitflip2 = rd64(s, 40) ^ rd64(s, 48);
            let lo = rd64(input, 0) ^ bitflip1;
            let hi = rd64(input, len - 8) ^ bitflip2;
            let acc = (len as u64).wrapping_add(lo.swap_bytes()).wrapping_add(hi).wrapping_add(mul128_fold64(lo, hi));
            return xxh3_avalanche(acc);
        }
        if len >= 4 {
            let bitflip = rd64(s, 8) ^ rd64(s, 16);
            let input64 = rd32(input, len - 4).wrapping_add(rd32(input, 0) << 32);
            return xxh3_rrmxmx(input64 ^ bitflip, len as u64);
        }
        if len > 0 {
            let c1 = input[0] as u32;
            let c2 = input[len >> 1] as u32;
            let c3 = input[len - 1] as u32;
            let combined = (c1 << 16) | (c2 << 24) | c3 | ((len as u32) << 8);
            let bitflip = rd32(s, 0) ^ rd32(s, 4);
            return xxh64_avalanche((combined as u64) ^ bitflip);
        }
        return xxh64_avalanche(rd64(s, 56) ^ rd64(s, 64));
    }
    if len <= 128 {
        let mut acc = (len as u64).wrapping_mul(XXH_PRIME64_1);
        let mut i = (len - 1) / 32;
        loop {
            acc = acc.wrapping_add(xxh3_mix16b(input, 16 * i, s, 32 * i));
            acc = acc.wrapping_add(xxh3_mix16b(input, len - 16 * (i + 1), s, 32 * i + 16));
            if i == 0 {
                break;
            }
            i -= 1;
        }
        return xxh3_avalanche(acc);
    }
    if len <= 240 {
        let mut acc = (len as u64).wrapping_mul(XXH_PRIME64_1);
        let nb_rounds = len / 16;
        for i in 0..8 {
            acc = acc.wrapping_add(xxh3_mix16b(input, 16 * i, s, 16 * i));
        }
        let mut acc_end = xxh3_mix16b(input, len - 16, s, 136 - 17);
        acc = xxh3_avalanche(acc);
        for i in 8..nb_rounds {
            acc_end = acc_end.wrapping_add(xxh3_mix16b(input, 16 * i, s, 16 * (i - 8) + 3));
        }
        return xxh3_avalanche(acc.wrapping_add(acc_end));
    }
    let mut acc: [u64; 8] = [
        XXH_PRIME32_3, XXH_PRIME64_1, XXH_PRIME64_2, XXH_PRIME64_3, XXH_PRIME64_4, XXH_PRIME32_2, XXH_PRIME64_5, XXH_PRIME32_1,
    ];
    let stripes_per_block = (192 - 64) / 8;
    let block_len = 64 * stripes_per_block;
    let nb_blocks = (len - 1) / block_len;
    for n in 0..nb_blocks {
        for st in 0..stripes_per_block {
            xxh3_accumulate_512(&mut acc, input, n * block_len + st * 64, s, st * 8);
        }
        xxh3_scramble(&mut acc, s, 192 - 64);
    }
    let nb_stripes = ((len - 1) - block_len * nb_blocks) / 64;
    for st in 0..nb_stripes {
        xxh3_accumulate_512(&mut acc, input, nb_blocks * block_len + st * 64, s, st * 8);
    }
    xxh3_accumulate_512(&mut acc, input, len - 64, s, 192 - 64 - 7);
    let mut result = (len as u64).wrapping_mul(XXH_PRIME64_1);
    for i in 0..4 {
        result = result.wrapping_add(mul128_fold64(acc[2 * i] ^ rd64(s, 11 + 16 * i), acc[2 * i + 1] ^ rd64(s, 11 + 16 * i + 8)));
    }
    xxh3_avalanche(result)
}

/// Psalm's `Interner::hash`: the xxh3 digest's 8 big-endian bytes read as a little-endian signed 64-bit
/// integer (`unpack('q', hash('xxh3', $s, true))`) with the sign bit cleared.
pub fn str_id(input: &[u8]) -> i64 {
    (xxh3_64(input).swap_bytes() as i64) & i64::MAX
}

#[cfg(test)]
mod xxh3_tests {
    // hash('xxh3', $s) for $s of length L with bytes ($i*73 + $L*31 + 7) % 256, generated by PHP 8.5
    const EXPECTED: [u64; 301] = [0x2d06800538d394c2u64, 0xc11d5b404d018be6u64, 0xb1ba1344e16d6f4du64, 0x283f516aebcdba4du64, 0x3f2d27b14c62e003u64, 0x5c9d489738cb4469u64, 0xc1ee7322f08d6c38u64, 0x36604b017dfaf202u64, 0x271e7b055bfaeaebu64, 0xcdb997e426997702u64, 0xff6402f42e587295u64, 0x4b1bcc7f666e3e1eu64, 0x6e56e8eba76aad54u64, 0xa82882b530860ed0u64, 0xef45d68aec1a1788u64, 0x57dfc4b1a854358bu64, 0x50fef4266fefd9c2u64, 0xd65e12184ed8d28eu64, 0x1f978a25682c0939u64, 0x8a1a9388e01822deu64, 0xdf6ab280fc98a0e5u64, 0x3142fd2887f2ce7fu64, 0xf7b8e52b50dfa51bu64, 0x20f1bfef0b25d11bu64, 0xf55a43a0d8119f20u64, 0x4af9505f1bf8d936u64, 0x8389a7c8c3a830deu64, 0xc97323bc0eaaba6du64, 0x7ccc8534d874be6eu64, 0x5de048a8719ae860u64, 0x456c26a993bf4bb1u64, 0x2ed5feb20d68f892u64, 0xc332e258fc1a3851u64, 0x8c38d59f9055621fu64, 0x24cc0ed65aacbb6cu64, 0xa27dd9947495f557u64, 0x03cb62e43bd414fbu64, 0x09c28f4f2d30b613u64, 0xfaa9d7b7809009cau64, 0x6f555dca3e18fbc5u64, 0xc8dc91dc993727a0u64, 0xb04093c5a601c474u64, 0x6b03fbebc561e417u64, 0x6b73866c8ad80c45u64, 0x39b7ab9855c25f2au64, 0x8e04ebcb69680a8bu64, 0x2980a497c6472cd6u64, 0x545521491ecbdc6au64, 0xc09d038d9cb7616du64, 0x6a5ae512974069b2u64, 0xa86b484a2894fb07u64, 0xe9962417eef07baeu64, 0x7324c213283ef3b5u64, 0x13a79204a911071eu64, 0xb5bec8ff59e7516fu64, 0x061b5d66134b760eu64, 0x79e47914245daebcu64, 0xca21888118767e0du64, 0x4a8eaaf95e902e82u64, 0xec8c5e71a8876f35u64, 0x1f16c03b55c9e37bu64, 0x62ae6850eec3817du64, 0x02cf9da2d58bea14u64, 0xf5b856d904eeb55fu64, 0x27db306ccf7b0a46u64, 0xcbf9ca2362075249u64, 0x5570078854da3403u64, 0xb017d937c9109af0u64, 0xfe7221ef561c94a5u64, 0x7e6d3914d52e60c3u64, 0xf45a224a5e030d52u64, 0xf1626b23c54477f2u64, 0x6a1c259cdbd35fdau64, 0x6bf0bc3b3470a1c8u64, 0x0917d3c076ad5ca1u64, 0x889bb90553cce240u64, 0x6829fc829cad530bu64, 0xb68e999bb6e12f4bu64, 0x968755613259a0a3u64, 0x9a48f5fa6fdfe1adu64, 0xeb79d6fae379bba8u64, 0x9d77e0053f99bc87u64, 0xa5d15f477567fec8u64, 0x9b6d9d1fa354e57cu64, 0x0002973d285f868eu64, 0xbd2f732e5c1bfe6eu64, 0xc6076a1207c6b4e2u64, 0x489db97994744162u64, 0x4178d3ad3546206du64, 0xc4086fd2aa40d297u64, 0xb8c5ba725426383au64, 0xecb83c35acdae9b7u64, 0xb2c033164dc53de5u64, 0x7a15577a17569df3u64, 0xd3215d797a0f86b2u64, 0xdd9921c9aed90163u64, 0xc73efdd025d979eau64, 0x8543f08e7b5e6c28u64, 0x9b6cd1c4a301e087u64, 0xddb879506b4f4dc7u64, 0x63f12489c5fe3a00u64, 0x009dada41a7379afu64, 0xce127e8d1c39385au64, 0x85aa03f6ef88b557u64, 0xf361dd368c329b76u64, 0x92fa3a5a1c5c654fu64, 0x467d439db31ff4c0u64, 0xd7d48af8e1d40cb7u64, 0x154d7f5ca43e3b2eu64, 0xe3f8a71b0d6f9280u64, 0x4c3643b60d34a9f4u64, 0x819d8fd7a85a8b97u64, 0x5077e04720accb3eu64, 0x85fb6dc22bfe3a21u64, 0x0814608fad390c1bu64, 0x4536d5a6d9da6f85u64, 0x06290507cafec7bdu64, 0xa5c8ac527a1ca4bbu64, 0xd0aa67d0c4cffb99u64, 0x0fc16bc5d320b953u64, 0x0d2f3854e9057c7bu64, 0x4b509039815bf3feu64, 0x1ec295d1d1eb66fau64, 0x01fd42cfd1cccab0u64, 0x5dde621a630c8a12u64, 0xc7e02ae3938905f7u64, 0x0d2496f3525913c5u64, 0xac71ccdd59eeb685u64, 0x245f0f5157d2569eu64, 0x4ef38a6e0e1f8ff6u64, 0xd9cb9487b8f4ae09u64, 0xae422a50cb48cdbeu64, 0xabc5cfe15027a218u64, 0xf52736ba34dffa96u64, 0xc26f8bea7ef1fbacu64, 0x4cd8a08bcb4c67a9u64, 0xc9635d8aed58390fu64, 0x2ff4414fa2c46672u64, 0xe9c136ce997181f3u64, 0x3bc424356d8ef75bu64, 0x2c5c8daed7d98542u64, 0x07e409bf02d78636u64, 0x853760fc9dfa842bu64, 0xcd5e51a1c3321317u64, 0x6355a42c380b68e4u64, 0x6f013afdffaba959u64, 0xdda94e251b5c4644u64, 0xa9f0256ec2077e34u64, 0x81567c400df23990u64, 0xbb547a692736bdbau64, 0xb22c292540206894u64, 0xb3e5226198d3a2feu64, 0xb8d9d5a97b350ff3u64, 0xaa025f6123d3c6e6u64, 0xfb758d9c897103ceu64, 0xe7a96030d88ec1ccu64, 0x864291fbf22a30e0u64, 0xebcde452740712f7u64, 0xfe9ca92e2513a884u64, 0xd6a360ab6b16dd96u64, 0xd331c591b5d2f284u64, 0x7cac4d195dc276f0u64, 0x6e2b2f945f65f8c2u64, 0x9bbc73a77272f83cu64, 0x42bf75f8efed5c4bu64, 0x35e65416bfefb15au64, 0x60b97fc3202a8c17u64, 0x640c8fb90cdc87b9u64, 0xc2120c2c3b6357c7u64, 0x554a2a00c2803ac7u64, 0x8f4fd55e33a9d678u64, 0x3d5a3b09c8963659u64, 0x68d995c9271288d1u64, 0x31b363f454039e19u64, 0xd568ab087bb3805au64, 0x90542972d3ae550cu64, 0x3081d7dece6c4824u64, 0xc19e9b677f5267c8u64, 0x28664f38551be334u64, 0x0d79f9af9cab7247u64, 0xb200a978c2cc7ebfu64, 0x5b0579e049ca9a90u64, 0x1734a439a4576881u64, 0xc8d8a5b509c60f0eu64, 0x8a36cf79538d66d0u64, 0x3294dd109e9b3c7eu64, 0x57294eb6f9ddc9bau64, 0x8cf316b8706b973bu64, 0x34782cab6006ff66u64, 0x9c2eaef7a51d9d1cu64, 0xbe8497d87266d693u64, 0x26ed336f027469c9u64, 0xe9887bd64a260c51u64, 0x8a01dc780dd8a91fu64, 0xca41fa2e95bc8a80u64, 0xb751f2467e6f2ae5u64, 0x1e84ed8ce350c636u64, 0x6dfd9dcb97326956u64, 0x93558fad57abc5f7u64, 0xc71fa040719866fcu64, 0xf394237eff10b4fau64, 0xc31e6d4809472304u64, 0x33107893a5dadc79u64, 0xed1a7bf2c6fe5a53u64, 0x8141e1fcf5e9aea3u64, 0xecf81424af7642b4u64, 0xfe7f8d1797afda62u64, 0xebdc0b67d3ad1ddeu64, 0x5a40421ef3878a2au64, 0x49670a14f7907690u64, 0xdecdac15633294eeu64, 0xbae94fd532d224c4u64, 0x7e4ec8ea1b8800cau64, 0x3293652209d526e3u64, 0x0c4ef341c9db1467u64, 0xdcb823f680188a1fu64, 0xfe254e860dca875eu64, 0x8caf5645427b09f5u64, 0x0cb0f9eb563d3404u64, 0x5364bb5afd42ab32u64, 0x6ea6aa659795538bu64, 0x4c564fe3e2aa0339u64, 0xebfd4cdc09a35bd1u64, 0x23cfd275ae3886f9u64, 0x7e6f18e83090b168u64, 0x4109de39a73e419eu64, 0x699cb5830483408cu64, 0x8597e169bc1727ceu64, 0xc3cc60d914815fbfu64, 0x9b3b44ca7181e700u64, 0x2062a2472f8161b3u64, 0x65d7cd9ef037159bu64, 0x9bf064f696739757u64, 0xfc9edf71f725c618u64, 0x0e5306e62a883194u64, 0xa8c26419b2a9d306u64, 0x1b61bd14a26b05a4u64, 0x8361a3d4b16f4d69u64, 0xc26b410d01d8af59u64, 0x125855236691126fu64, 0xc99f17babc7ef124u64, 0xbc05c856b9e69f2eu64, 0x37f4d01c99009eeau64, 0x9808dd71ee91a6eeu64, 0x993a0686969fe76bu64, 0x52359211d6db80b9u64, 0x680f0c89d8067664u64, 0x86e8407ee7315713u64, 0x88cef4292c0e9e56u64, 0x554f374f28996edbu64, 0x7f65ec3392757445u64, 0x324a074ecc1ee38au64, 0xee02a358dbc6f110u64, 0xb256b007e2bcd3a7u64, 0xcac8f8f04d63b1b4u64, 0xf4f51db58c1ada83u64, 0xeb787974a8645058u64, 0x477a335db6de8712u64, 0x1b388daaa949a7fau64, 0x445de322270d3c31u64, 0x51227eb7761b1306u64, 0x02025ec59fac6d14u64, 0x369729d1b3df13b5u64, 0xf9ffdef9b7591b2eu64, 0x8f0c2aec54dee70au64, 0x988094d5ab97349eu64, 0x0ad3435c98cbb04du64, 0xf6136ca35585015cu64, 0x74cab6d5ea14eb8cu64, 0x7b72a44696573a2du64, 0x25291fb76d42ca8au64, 0x70ca56cd6ef758f9u64, 0x6f280e6e6eef4208u64, 0xd7fdb1f66cb4fb98u64, 0xe8862fd569c8b34bu64, 0xda90bead58a8c653u64, 0xdb3509205af45c59u64, 0x8c44e47fa5c5de23u64, 0x0f188fa9e9a7d203u64, 0xbcc62cad790670beu64, 0x5b110c37a5798525u64, 0x3d27da34db80eb98u64, 0x85ca689d1196726eu64, 0xde3a0b482160ef82u64, 0x147394ce877090e2u64, 0xed1a1db16166dac7u64, 0x7d24d56e1042d64au64, 0x5bfd5c86978ae02bu64, 0x725a4800b4058f4bu64, 0x00b4db2be11fdd1bu64, 0xaae59f55d1f20b84u64, 0x4e958e0acd0c612cu64, 0x33fc306dc63cab0eu64, 0x2a323addd81236dau64, 0x890cc0c8a883e453u64, 0x2f7390ea1fdfcce3u64, 0xdde7e8c2b644d91du64, 0x2661adcb86f04553u64, 0xbd8c68e87b70727bu64, 0x00e814324193c3f0u64, 0xdbf6ca43e70fb846u64];
    const EXPECTED_IDS: [i64; 301] = [4797691740620326445, 7389001044935253441, 5579799277612546737, 5601015497573351208, 279331258426142015, 7585411116315483484, 4065780626118405825, 212507547239407670, 7776302975449505319, 177779102627379661, 1545394580827038975, 2179300656203701067, 6101650339606845038, 5768695716469942440, 582964280011671023, 807644791696580439, 4817144537123454544, 1068153891563134678, 4109864960770348831, 6783011341416471178, 7323021204362783455, 9209564950341435953, 1992243945608165623, 2004424042353783072, 2350617152610065141, 3952462944441268554, 6787110197285849475, 7906819076065031113, 7979944061690039420, 6983001035183284317, 3552143372030733381, 1366956894953854254, 5852457386893259459, 2261464042538612876, 7835045482879437860, 6338136378539867554, 8868946919899187971, 1420375703648453129, 5334954116111641082, 5042650865733555567, 2316881667275480264, 8413851819625889968, 1721608460257919851, 4975589777375392619, 3053372745269884729, 795563087786345614, 6209416904510308393, 7700252793833084244, 7881782705883684288, 3632505592052275818, 575216278185733032, 3349535653261252329, 3887519245201974387, 2163717563598677779, 8021446785053015733, 1042102910476688134, 4372534697767068793, 972344417183080906, 157221774594575946, 3850445363951340780, 8927200254290302495, 9043724955055337058, 1507170774981398274, 6896680108981401845, 5046982463225649959, 5283293430947707339, 231049537497034837, 8113816135072815024, 2707820435661222654, 4854931891007614334, 5912385589312903924, 8248136856045839089, 6512156526372002922, 5233587611624075371, 2403987026661807881, 4675524020114398088, 816186845208258920, 5417797051160825526, 2567149860517414806, 3306169273511397530, 2935073602234907115, 557488949847685021, 5259755168221614501, 8999692489956617627, 1046628589846856192, 7997860070986821565, 7112527445200865222, 7080068270249319752, 7863362145752414273, 1716505510400690372, 4195145197039568312, 4028991774822021356, 7295203906285715634, 8330909544542049658, 3640614367472198099, 7134222629453535709, 7672402196645625543, 2912806943773967237, 567455355942038683, 5137850072229853405, 16605673012130147, 3420891950233787648, 6501008856905028302, 6320108216290355845, 8546480295662936563, 5721080178486737554, 4680400769042840902, 3966779438809142487, 3331325225018019093, 41217448689006819, 8406307412960818764, 1696549365852642689, 4524899505268619088, 2394505615799876485, 1948996156342146056, 391772320946271813, 4451806900333652230, 4297591256116938917, 1872318514319567568, 6032889269761851663, 8897993463475744525, 9147755878694277195, 8819996206648640030, 3515847660613598465, 1335893859252756061, 8576412333661413575, 4977420225983423501, 411778485691642284, 2186165941598904100, 8543081164989723470, 697764065674841049, 4525353238637724334, 1775024431476753835, 1655881230567483381, 3241449884226383810, 2983437715260889164, 1097005761493296073, 8243492369341740079, 8323058491761017321, 6626921976393155643, 4793476898775456812, 3929064131722339335, 3135906797007681413, 1662728503236320973, 7235045137265743203, 6460884256393134447, 4919720914608630237, 3782469268690825385, 1169231717009741441, 4232598767699383483, 1470460738209918130, 9125088446424212915, 8290904244258986424, 7405838786168292010, 5621461595739485691, 5530858876563401191, 6931087049614246534, 8579928436282543595, 335539222656687358, 1647497690380018646, 356578997792223699, 8103878284612054140, 4825718461909904238, 4393387273261137051, 5430476865623408450, 6535268138283755061, 1696777479819082080, 4145523929228708964, 5140686605459919554, 5132556296056883797, 8707332967666896783, 6428491304010471997, 5874965675999877480, 1845916560680792881, 6521409601440278741, 888808724652184720, 2614459219446300976, 5217229400484585153, 3810919762516469288, 5148366013493508365, 4575319407075066034, 1196491069667738971, 101427354469151767, 1013246186186201288, 5793473360324146826, 9096316450571654194, 4236160989093833047, 4294018901590078348, 7421657723188312116, 2061836803067752092, 1429442575182824638, 5289886791127788838, 5840084919660546281, 2281592238613725578, 39050895710372298, 7289761534999417271, 3946931061688992798, 6226563587354000749, 8630492653987779987, 8964019720510513095, 8841710657954092275, 298160105531973315, 8781133778144202803, 6006393183477111533, 2571249679994012033, 3765702733031864556, 7123198824109932542, 6781767737368370411, 3065411974633504858, 1186294942726252361, 7968049041872047582, 4910280609509665210, 5332411612291157630, 7144632094078374706, 7427803342882164236, 2272655904126253276, 6811635120857818622, 8433407301789331340, 302934518476156940, 3651085578830308435, 816160434962474606, 4108315176544982604, 5862458602698898923, 8756748847420722979, 7543969390549036926, 2180092582575343937, 882849582377442409, 5631495957047777157, 4566510473723301059, 65163045344918427, 3702382408740528672, 1951527520130946917, 6311640494999138459, 1785156046996610812, 1455093872446558990, 491923366708822696, 2595599104125788443, 7587843756756525443, 6462621489328909250, 8003619355656280082, 2662048203793735625, 3359657530740966844, 7682578671954097207, 7973220643604596888, 7775358750520916633, 4143553369645135186, 7238980978126622568, 1393637480019847302, 6241441716427345544, 6588371703296380757, 5004754356712269183, 784504622923860530, 1220975619381330670, 2869845066542765746, 3796925147256506570, 277563518673090036, 6363696948145912043, 1335280689367710279, 8838113786797570075, 3547725068201319748, 437723785483657809, 1472022455431070210, 3824646471968986934, 3322347796821901305, 785841116320238735, 2176531284205273240, 5598198192064942858, 6629726729326564342, 930860746822568564, 3259013583297868411, 777506924665973029, 8744010731803298416, 595301358338189423, 1800230856152972759, 5454923930549585640, 6036697449362264282, 6439290235154675163, 2584720550763185292, 275467149236967439, 4499103147409327804, 2703700901805953371, 1795670555410179901, 7958588493916588677, 211493403568585438, 7102300340952003348, 5177563291385141997, 5392570242169119869, 3161679321813024091, 5444576745171212914, 2007796060841030656, 291593581794157994, 3197851285194708302, 1057005360481631283, 6500403834789704234, 6045101360783821961, 7191368033176154927, 2150825848790902749, 6000466540274475302, 8895295891492605117, 8125500060928894976, 5095840464318953179];

    #[test]
    fn matches_php() {
        for (l, (want, want_id)) in EXPECTED.iter().zip(EXPECTED_IDS.iter()).enumerate() {
            let s: Vec<u8> = (0..l).map(|i| ((i * 73 + l * 31 + 7) % 256) as u8).collect();
            assert_eq!(super::xxh3_64(&s), *want, "xxh3 of length {}", l);
            assert_eq!(super::str_id(&s), *want_id, "id of length {}", l);
        }
    }
}

#[cfg(test)]
mod xxh3_tests {
    use super::xxh3_64;
    #[test]
    fn matches_php_hash_xxh3() {
        // php -r 'echo hash("xxh3", $s);'
        assert_eq!(format!("{:016x}", xxh3_64(b"")), "2d06800538d394c2");
        assert_eq!(format!("{:016x}", xxh3_64(b"abc")), "78af5f94892f3950");
        assert_eq!(format!("{:016x}", xxh3_64(&[b'x'; 17])), "89975e6b7d2f5a11");
        assert_eq!(format!("{:016x}", xxh3_64(&b"ab".repeat(70))), "9661f38ded3e7931");
        assert_eq!(format!("{:016x}", xxh3_64(&[b'q'; 300])), "3c634963c2a92b89");
    }
}
