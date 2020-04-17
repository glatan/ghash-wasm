use std::cmp::Ordering;

use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
#[cfg(debug_assertions)]
use crate::*;

// K(t) = 5A827999 ( 0 <= t <= 19)
// K(t) = 6ED9EBA1 (20 <= t <= 39)
// K(t) = 8F1BBCDC (40 <= t <= 59)
// K(t) = CA62C1D6 (60 <= t <= 79)
const K: [u32; 4] = [0x5A82_7999, 0x6ED9_EBA1, 0x8F1B_BCDC, 0xCA62_C1D6];

const H: [u32; 5] = [
    0x6745_2301,
    0xEFCD_AB89,
    0x98BA_DCFE,
    0x1032_5476,
    0xC3D2_E1F0,
];

// 0 <= t <= 19
const fn ch(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}

// 20 <= t <= 39, 60 <= t <= 79
const fn parity(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

// 40 <= t <= 59
const fn maj(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (b & d) | (c & d)
}

#[wasm_bindgen]
pub struct Sha1 {
    input: Vec<u8>,
    word_block: Vec<u32>,
    status: [u32; 5],
}

#[wasm_bindgen]
impl Sha1 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            word_block: Vec::new(),
            status: H,
        }
    }
    fn padding(&mut self) {
        let input_length = self.input.len();
        // word_block末尾に0x80を追加(0b1000_0000)
        self.input.push(0x80);
        // [byte]: 64 - 8(input_length) - 1(0x80) = 55
        let padding_length = 55 - (input_length as i128);
        match padding_length.cmp(&0) {
            Ordering::Greater => {
                self.input.append(&mut vec![0; padding_length as usize]);
            }
            Ordering::Less => {
                self.input
                    .append(&mut vec![0; 64 - (padding_length.abs() % 64) as usize]);
            }
            Ordering::Equal => (),
        }
        // 入力データの長さを追加
        self.input
            .append(&mut (8 * input_length as u64).to_be_bytes().to_vec());
        // iは4の倍数となる (0, 4, 8..60..)
        for i in (0..self.input.len()).filter(|i| i % 4 == 0) {
            self.word_block.push(u32::from_be_bytes([
                self.input[i],
                self.input[i + 1],
                self.input[i + 2],
                self.input[i + 3],
            ]));
        }
    }
    #[allow(clippy::many_single_char_names, clippy::needless_range_loop)]
    fn round(&mut self) {
        let (mut a, mut b, mut c, mut d, mut e);
        let mut temp;
        let mut w = [0; 80];
        for i in 0..(self.word_block.len() / 16) {
            for t in 0..16 {
                w[t] = self.word_block[t + i * 16];
            }
            for t in 16..80 {
                w[t] = (w[t - 3] ^ w[t - 8] ^ w[t - 14] ^ w[t - 16]).rotate_left(1);
            }
            a = self.status[0];
            b = self.status[1];
            c = self.status[2];
            d = self.status[3];
            e = self.status[4];
            // 0 <= t <= 19
            for wt in w.iter().take(20) {
                temp = a
                    .rotate_left(5)
                    .wrapping_add(ch(b, c, d))
                    .wrapping_add(e)
                    .wrapping_add(*wt)
                    .wrapping_add(K[0]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }
            // 20 <= t <= 39
            for wt in w.iter().take(40).skip(20) {
                temp = a
                    .rotate_left(5)
                    .wrapping_add(parity(b, c, d))
                    .wrapping_add(e)
                    .wrapping_add(*wt)
                    .wrapping_add(K[1]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }
            // 40 <= t <= 59
            for wt in w.iter().take(60).skip(40) {
                temp = a
                    .rotate_left(5)
                    .wrapping_add(maj(b, c, d))
                    .wrapping_add(e)
                    .wrapping_add(*wt)
                    .wrapping_add(K[2]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }
            // 60 <= t <= 79
            for wt in w.iter().skip(60) {
                temp = a
                    .rotate_left(5)
                    .wrapping_add(parity(b, c, d))
                    .wrapping_add(e)
                    .wrapping_add(*wt)
                    .wrapping_add(K[3]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }
            self.status[0] = self.status[0].wrapping_add(a);
            self.status[1] = self.status[1].wrapping_add(b);
            self.status[2] = self.status[2].wrapping_add(c);
            self.status[3] = self.status[3].wrapping_add(d);
            self.status[4] = self.status[4].wrapping_add(e);
        }
    }
    fn hash(input: &[u8]) -> Vec<u8> {
        let mut sha1 = Self::new();
        sha1.input = input.to_vec();
        sha1.padding();
        sha1.round();
        sha1.status
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .collect()
    }
    #[wasm_bindgen]
    pub fn hash_to_lowercase(input: &[u8]) -> String {
        Self::hash(input)
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}
