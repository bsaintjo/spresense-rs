// CLEFIA-128 block cipher + CMAC, ported from nuttx/tools/cxd56/clefia.c
// SPDX-License-Identifier: Apache-2.0
//
// Original work: Copyright (c) Apache Software Foundation / Sony Semiconductor Solutions Corporation
// This file has been modified from the original C source (ported to Rust).

// S0: 8-bit S-box based on four 4-bit S-boxes
static S0: [u8; 256] = [
    0x57, 0x49, 0xd1, 0xc6, 0x2f, 0x33, 0x74, 0xfb, 0x95, 0x6d, 0x82, 0xea, 0x0e, 0xb0, 0xa8, 0x1c,
    0x28, 0xd0, 0x4b, 0x92, 0x5c, 0xee, 0x85, 0xb1, 0xc4, 0x0a, 0x76, 0x3d, 0x63, 0xf9, 0x17, 0xaf,
    0xbf, 0xa1, 0x19, 0x65, 0xf7, 0x7a, 0x32, 0x20, 0x06, 0xce, 0xe4, 0x83, 0x9d, 0x5b, 0x4c, 0xd8,
    0x42, 0x5d, 0x2e, 0xe8, 0xd4, 0x9b, 0x0f, 0x13, 0x3c, 0x89, 0x67, 0xc0, 0x71, 0xaa, 0xb6, 0xf5,
    0xa4, 0xbe, 0xfd, 0x8c, 0x12, 0x00, 0x97, 0xda, 0x78, 0xe1, 0xcf, 0x6b, 0x39, 0x43, 0x55, 0x26,
    0x30, 0x98, 0xcc, 0xdd, 0xeb, 0x54, 0xb3, 0x8f, 0x4e, 0x16, 0xfa, 0x22, 0xa5, 0x77, 0x09, 0x61,
    0xd6, 0x2a, 0x53, 0x37, 0x45, 0xc1, 0x6c, 0xae, 0xef, 0x70, 0x08, 0x99, 0x8b, 0x1d, 0xf2, 0xb4,
    0xe9, 0xc7, 0x9f, 0x4a, 0x31, 0x25, 0xfe, 0x7c, 0xd3, 0xa2, 0xbd, 0x56, 0x14, 0x88, 0x60, 0x0b,
    0xcd, 0xe2, 0x34, 0x50, 0x9e, 0xdc, 0x11, 0x05, 0x2b, 0xb7, 0xa9, 0x48, 0xff, 0x66, 0x8a, 0x73,
    0x03, 0x75, 0x86, 0xf1, 0x6a, 0xa7, 0x40, 0xc2, 0xb9, 0x2c, 0xdb, 0x1f, 0x58, 0x94, 0x3e, 0xed,
    0xfc, 0x1b, 0xa0, 0x04, 0xb8, 0x8d, 0xe6, 0x59, 0x62, 0x93, 0x35, 0x7e, 0xca, 0x21, 0xdf, 0x47,
    0x15, 0xf3, 0xba, 0x7f, 0xa6, 0x69, 0xc8, 0x4d, 0x87, 0x3b, 0x9c, 0x01, 0xe0, 0xde, 0x24, 0x52,
    0x7b, 0x0c, 0x68, 0x1e, 0x80, 0xb2, 0x5a, 0xe7, 0xad, 0xd5, 0x23, 0xf4, 0x46, 0x3f, 0x91, 0xc9,
    0x6e, 0x84, 0x72, 0xbb, 0x0d, 0x18, 0xd9, 0x96, 0xf0, 0x5f, 0x41, 0xac, 0x27, 0xc5, 0xe3, 0x3a,
    0x81, 0x6f, 0x07, 0xa3, 0x79, 0xf6, 0x2d, 0x38, 0x1a, 0x44, 0x5e, 0xb5, 0xd2, 0xec, 0xcb, 0x90,
    0x9a, 0x36, 0xe5, 0x29, 0xc3, 0x4f, 0xab, 0x64, 0x51, 0xf8, 0x10, 0xd7, 0xbc, 0x02, 0x7d, 0x8e,
];

// S1: 8-bit S-box based on inverse function
static S1: [u8; 256] = [
    0x6c, 0xda, 0xc3, 0xe9, 0x4e, 0x9d, 0x0a, 0x3d, 0xb8, 0x36, 0xb4, 0x38, 0x13, 0x34, 0x0c, 0xd9,
    0xbf, 0x74, 0x94, 0x8f, 0xb7, 0x9c, 0xe5, 0xdc, 0x9e, 0x07, 0x49, 0x4f, 0x98, 0x2c, 0xb0, 0x93,
    0x12, 0xeb, 0xcd, 0xb3, 0x92, 0xe7, 0x41, 0x60, 0xe3, 0x21, 0x27, 0x3b, 0xe6, 0x19, 0xd2, 0x0e,
    0x91, 0x11, 0xc7, 0x3f, 0x2a, 0x8e, 0xa1, 0xbc, 0x2b, 0xc8, 0xc5, 0x0f, 0x5b, 0xf3, 0x87, 0x8b,
    0xfb, 0xf5, 0xde, 0x20, 0xc6, 0xa7, 0x84, 0xce, 0xd8, 0x65, 0x51, 0xc9, 0xa4, 0xef, 0x43, 0x53,
    0x25, 0x5d, 0x9b, 0x31, 0xe8, 0x3e, 0x0d, 0xd7, 0x80, 0xff, 0x69, 0x8a, 0xba, 0x0b, 0x73, 0x5c,
    0x6e, 0x54, 0x15, 0x62, 0xf6, 0x35, 0x30, 0x52, 0xa3, 0x16, 0xd3, 0x28, 0x32, 0xfa, 0xaa, 0x5e,
    0xcf, 0xea, 0xed, 0x78, 0x33, 0x58, 0x09, 0x7b, 0x63, 0xc0, 0xc1, 0x46, 0x1e, 0xdf, 0xa9, 0x99,
    0x55, 0x04, 0xc4, 0x86, 0x39, 0x77, 0x82, 0xec, 0x40, 0x18, 0x90, 0x97, 0x59, 0xdd, 0x83, 0x1f,
    0x9a, 0x37, 0x06, 0x24, 0x64, 0x7c, 0xa5, 0x56, 0x48, 0x08, 0x85, 0xd0, 0x61, 0x26, 0xca, 0x6f,
    0x7e, 0x6a, 0xb6, 0x71, 0xa0, 0x70, 0x05, 0xd1, 0x45, 0x8c, 0x23, 0x1c, 0xf0, 0xee, 0x89, 0xad,
    0x7a, 0x4b, 0xc2, 0x2f, 0xdb, 0x5a, 0x4d, 0x76, 0x67, 0x17, 0x2d, 0xf4, 0xcb, 0xb1, 0x4a, 0xa8,
    0xb5, 0x22, 0x47, 0x3a, 0xd5, 0x10, 0x4c, 0x72, 0xcc, 0x00, 0xf9, 0xe0, 0xfd, 0xe2, 0xfe, 0xae,
    0xf8, 0x5f, 0xab, 0xf1, 0x1b, 0x42, 0x81, 0xd6, 0xbe, 0x44, 0x29, 0xa6, 0x57, 0xb9, 0xaf, 0xf2,
    0xd4, 0x75, 0x66, 0xbb, 0x68, 0x9f, 0x50, 0x02, 0x01, 0x3c, 0x7f, 0x8d, 0x1a, 0x88, 0xbd, 0xac,
    0xf7, 0xe4, 0x79, 0x96, 0xa2, 0xfc, 0x6d, 0xb2, 0x6b, 0x03, 0xe1, 0x2e, 0x7d, 0x14, 0x95, 0x1d,
];

// GF(2^8) multiply-by-2; reduction polynomial x^8 + x^4 + x^3 + x^2 + 1 (0x11d)
fn mul2(x: u8) -> u8 {
    let x = if x & 0x80 != 0 { x ^ 0x0e } else { x };
    (x << 1) | (x >> 7)
}

fn mul4(x: u8) -> u8 {
    mul2(mul2(x))
}
fn mul6(x: u8) -> u8 {
    mul2(x) ^ mul4(x)
}
fn mul8(x: u8) -> u8 {
    mul2(mul4(x))
}
fn mula(x: u8) -> u8 {
    mul2(x) ^ mul8(x)
}

fn xor4(a: &[u8], b: &[u8]) -> [u8; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

// F0 round function (XOR into dst[4..8])
fn f0_xor(dst: &mut [u8], src: &[u8], rk: &[u8]) {
    let x = xor4(&src[..4], &rk[..4]);
    let z = [
        S0[x[0] as usize],
        S1[x[1] as usize],
        S0[x[2] as usize],
        S1[x[3] as usize],
    ];
    let y = [
        z[0] ^ mul2(z[1]) ^ mul4(z[2]) ^ mul6(z[3]),
        mul2(z[0]) ^ z[1] ^ mul6(z[2]) ^ mul4(z[3]),
        mul4(z[0]) ^ mul6(z[1]) ^ z[2] ^ mul2(z[3]),
        mul6(z[0]) ^ mul4(z[1]) ^ mul2(z[2]) ^ z[3],
    ];
    dst[..4].copy_from_slice(&src[..4]);
    for i in 0..4 {
        dst[4 + i] = src[4 + i] ^ y[i];
    }
}

// F1 round function (XOR into dst[4..8])
fn f1_xor(dst: &mut [u8], src: &[u8], rk: &[u8]) {
    let x = xor4(&src[..4], &rk[..4]);
    let z = [
        S1[x[0] as usize],
        S0[x[1] as usize],
        S1[x[2] as usize],
        S0[x[3] as usize],
    ];
    let y = [
        z[0] ^ mul8(z[1]) ^ mul2(z[2]) ^ mula(z[3]),
        mul8(z[0]) ^ z[1] ^ mula(z[2]) ^ mul2(z[3]),
        mul2(z[0]) ^ mula(z[1]) ^ z[2] ^ mul8(z[3]),
        mula(z[0]) ^ mul2(z[1]) ^ mul8(z[2]) ^ z[3],
    ];
    dst[..4].copy_from_slice(&src[..4]);
    for i in 0..4 {
        dst[4 + i] = src[4 + i] ^ y[i];
    }
}

// GFN_{4,r}: 4-branch generalized Feistel, r rounds
fn gfn4(x: &[u8; 16], rk: &[u8], r: usize) -> [u8; 16] {
    let mut fin = *x;
    let mut fout = [0u8; 16];
    let mut rk_off = 0;
    let mut round = r;
    loop {
        f0_xor(&mut fout[..8], &fin[..8], &rk[rk_off..]);
        f1_xor(&mut fout[8..], &fin[8..], &rk[rk_off + 4..]);
        rk_off += 8;
        round -= 1;
        if round == 0 {
            break;
        }
        // swap for encryption
        fin[..12].copy_from_slice(&fout[4..16]);
        fin[12..].copy_from_slice(&fout[..4]);
    }
    fout
}

fn double_swap(lk: &mut [u8; 16]) {
    let t: [u8; 16] = [
        (lk[0] << 7) | (lk[1] >> 1),
        (lk[1] << 7) | (lk[2] >> 1),
        (lk[2] << 7) | (lk[3] >> 1),
        (lk[3] << 7) | (lk[4] >> 1),
        (lk[4] << 7) | (lk[5] >> 1),
        (lk[5] << 7) | (lk[6] >> 1),
        (lk[6] << 7) | (lk[7] >> 1),
        (lk[7] << 7) | (lk[15] & 0x7f),
        (lk[8] >> 7) | (lk[0] & 0xfe),
        (lk[9] >> 7) | (lk[8] << 1),
        (lk[10] >> 7) | (lk[9] << 1),
        (lk[11] >> 7) | (lk[10] << 1),
        (lk[12] >> 7) | (lk[11] << 1),
        (lk[13] >> 7) | (lk[12] << 1),
        (lk[14] >> 7) | (lk[13] << 1),
        (lk[15] >> 7) | (lk[14] << 1),
    ];
    *lk = t;
}

fn conset(iv: [u8; 2], lk_count: usize) -> Vec<u8> {
    let mut con = vec![0u8; lk_count * 8];
    let mut t = iv;
    for i in 0..lk_count {
        let off = i * 8;
        con[off] = t[0] ^ 0xb7;
        con[off + 1] = t[1] ^ 0xe1;
        con[off + 2] = !((t[0] << 1) | (t[1] >> 7));
        con[off + 3] = !((t[1] << 1) | (t[0] >> 7));
        con[off + 4] = !t[0] ^ 0x24;
        con[off + 5] = !t[1] ^ 0x3f;
        con[off + 6] = t[1];
        con[off + 7] = t[0];
        if t[1] & 0x01 != 0 {
            t[0] ^= 0xa8;
            t[1] ^= 0x30;
        }
        let tmp = t[0] << 7;
        t[0] = (t[0] >> 1) | (t[1] << 7);
        t[1] = (t[1] >> 1) | tmp;
    }
    con
}

// Round-key buffer: 8 (WK0,WK1) + 9*16 (RKi) + 8 (WK2,WK3) = 160 bytes
// In the C code: rk[8*26+16] = rk[224] bytes
pub struct Cipher {
    rk: Vec<u8>,
    k1: [u8; 16],
    #[allow(dead_code)]
    k2: [u8; 16],
    vector: [u8; 16],
}

// Key schedule for 128-bit key; always returns 18 rounds
pub fn keyset(skey: &[u8; 16]) -> (Vec<u8>, usize) {
    // cubic root of 2
    let con128 = conset([0x42, 0x8a], 30);

    // GFN_{4,12} to derive L from K
    let skey_arr: [u8; 16] = *skey;
    let mut lk: [u8; 16] = gfn4(&skey_arr, &con128, 12);

    let mut rk = vec![0u8; 8 * 26 + 16];
    // WK0, WK1
    rk[..8].copy_from_slice(&skey[..8]);
    let mut rk_off = 8;
    for i in 0..9 {
        // RKi
        let con_off = i * 16 + 4 * 24;
        for j in 0..16 {
            rk[rk_off + j] = lk[j] ^ con128[con_off + j];
        }
        if i % 2 != 0 {
            for j in 0..16 {
                rk[rk_off + j] ^= skey[j];
            }
        }
        double_swap(&mut lk);
        rk_off += 16;
    }
    // WK2, WK3
    rk[rk_off..rk_off + 8].copy_from_slice(&skey[8..]);

    (rk, 18)
}

pub fn encrypt(pt: &[u8; 16], rk: &[u8], rounds: usize) -> [u8; 16] {
    let mut rin = *pt;
    // initial key whitening
    for i in 0..4 {
        rin[4 + i] ^= rk[i];
    }
    for i in 0..4 {
        rin[12 + i] ^= rk[4 + i];
    }
    let rout = gfn4(&rin, &rk[8..], rounds);
    let mut ct = rout;
    // final key whitening
    let wk_off = 8 + rounds * 8;
    for i in 0..4 {
        ct[4 + i] ^= rk[wk_off + i];
    }
    for i in 0..4 {
        ct[12 + i] ^= rk[wk_off + 4 + i];
    }
    ct
}

fn left_shift_one(inp: &[u8; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];
    let mut overflow = 0u8;
    for i in (0..16).rev() {
        out[i] = (inp[i] << 1) | overflow;
        overflow = (inp[i] >> 7) & 1;
    }
    out
}

fn gen_subkeys(rk: &[u8], rounds: usize) -> ([u8; 16], [u8; 16]) {
    let l = encrypt(&[0u8; 16], rk, rounds);
    let mut k1 = left_shift_one(&l);
    if l[0] & 0x80 != 0 {
        k1[15] ^= 0x87;
    }
    let mut k2 = left_shift_one(&k1);
    if k1[0] & 0x80 != 0 {
        k2[15] ^= 0x87;
    }
    (k1, k2)
}

impl Cipher {
    pub fn new(key: &[u8; 16]) -> Self {
        let (rk, rounds) = keyset(key);
        let (k1, k2) = gen_subkeys(&rk, rounds);
        Cipher {
            rk,
            k1,
            k2,
            vector: [0u8; 16],
        }
    }

    // data must be 16-byte aligned; returns None on misalignment
    pub fn calc_cmac(&mut self, data: &[u8]) -> Option<[u8; 16]> {
        if data.len() % 16 != 0 {
            return None;
        }
        let rounds = 18;
        let mut m = [0u8; 16];
        let mut p = data;
        while !p.is_empty() {
            for i in 0..16 {
                m[i] = self.vector[i] ^ p[i];
            }
            self.vector = encrypt(&m, &self.rk, rounds);
            p = &p[16..];
        }
        // final: cmac = E(m XOR k1)
        let mut tmp = [0u8; 16];
        for i in 0..16 {
            tmp[i] = m[i] ^ self.k1[i];
        }
        Some(encrypt(&tmp, &self.rk, rounds))
    }
}

pub fn calc_cmac(key: &[u8; 16], data: &[u8]) -> Option<[u8; 16]> {
    Cipher::new(key).calc_cmac(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Encrypt test vector — generated from the C reference in nuttx/tools/cxd56/clefia.c
    // (Sony's embedded CLEFIA variant; note: diverges from standard RFC 6114 test vector)
    // Key: 0123456789abcdef fedcba9876543210
    // PT:  0123456789abcdef fedcba9876543210
    // CT:  ab46c140bea1d770 da0c870fe7a222a4  (from C reference)
    #[test]
    fn encrypt_c_reference() {
        let key: [u8; 16] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
            0x32, 0x10,
        ];
        let pt: [u8; 16] = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
            0x32, 0x10,
        ];
        let expected: [u8; 16] = [
            0xab, 0x46, 0xc1, 0x40, 0xbe, 0xa1, 0xd7, 0x70, 0xda, 0x0c, 0x87, 0x0f, 0xe7, 0xa2,
            0x22, 0xa4,
        ];
        let (rk, rounds) = keyset(&key);
        let ct = encrypt(&pt, &rk, rounds);
        assert_eq!(ct, expected, "encrypt test vector failed");
    }

    // CMAC over 32 zero bytes under the SPK VMK key
    // CMAC(vmk, [0u8; 32]) = de2b5b8acefd7d60 8c5dc835f9c5d0c5  (from C reference)
    #[test]
    fn cmac_vmk_zeros32() {
        const VMK: [u8; 16] = [
            0x27, 0xc0, 0xaf, 0x1b, 0x5d, 0xcb, 0xc6, 0xc5, 0x58, 0x22, 0x1c, 0xdd, 0xaf, 0xf3,
            0x20, 0x21,
        ];
        let expected: [u8; 16] = [
            0xde, 0x2b, 0x5b, 0x8a, 0xce, 0xfd, 0x7d, 0x60, 0x8c, 0x5d, 0xc8, 0x35, 0xf9, 0xc5,
            0xd0, 0xc5,
        ];
        let data = [0u8; 32];
        let tag = calc_cmac(&VMK, &data).expect("32 bytes is 16-aligned");
        assert_eq!(tag, expected, "CMAC VMK zeros32 failed");
    }
}
