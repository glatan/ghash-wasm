use std::cmp::Ordering;

use wasm_bindgen::prelude::*;

#[allow(unused_imports)]
#[cfg(debug_assertions)]
use crate::*;

const WORD_BUFFER: [u32; 4] = [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];

#[allow(clippy::many_single_char_names)]
const fn round1(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
    const fn f(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (!x & z)
    }
    a.wrapping_add(f(b, c, d)).wrapping_add(k).rotate_left(s)
}

#[allow(clippy::many_single_char_names)]
const fn round2(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
    const fn g(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (x & z) | (y & z)
    }
    a.wrapping_add(g(b, c, d))
        .wrapping_add(k)
        .wrapping_add(0x5A82_7999)
        .rotate_left(s)
}

#[allow(clippy::many_single_char_names)]
const fn round3(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
    const fn h(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }
    a.wrapping_add(h(b, c, d))
        .wrapping_add(k)
        .wrapping_add(0x6ED9_EBA1)
        .rotate_left(s)
}

#[wasm_bindgen]
pub struct Md4 {
    input: Vec<u8>,
    word_block: Vec<u32>,
    status: [u32; 4],
}

#[wasm_bindgen]
impl Md4 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            word_block: Vec::new(),
            status: WORD_BUFFER,
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
            .append(&mut (8 * input_length as u64).to_le_bytes().to_vec());
        // iは4の倍数となる (0, 4, 8..60..)
        for i in (0..self.input.len()).filter(|i| i % 4 == 0) {
            self.word_block.push(u32::from_le_bytes([
                self.input[i],
                self.input[i + 1],
                self.input[i + 2],
                self.input[i + 3],
            ]));
        }
    }
    fn round(&mut self) {
        let word_block_length = self.word_block.len() / 16;
        let (mut a, mut b, mut c, mut d);
        let mut x: [u32; 16] = [0; 16];
        for i in 0..word_block_length {
            for j in 0..16 {
                x[j] = self.word_block[16 * i + j];
            }
            a = self.status[0];
            b = self.status[1];
            c = self.status[2];
            d = self.status[3];
            // Round 1
            for &k in &[0, 4, 8, 12] {
                a = round1(a, b, c, d, x[k], 3);
                d = round1(d, a, b, c, x[k + 1], 7);
                c = round1(c, d, a, b, x[k + 2], 11);
                b = round1(b, c, d, a, x[k + 3], 19);
            }
            // Round 2
            for k in 0..4 {
                a = round2(a, b, c, d, x[k], 3);
                d = round2(d, a, b, c, x[k + 4], 5);
                c = round2(c, d, a, b, x[k + 8], 9);
                b = round2(b, c, d, a, x[k + 12], 13);
            }
            // Round 3
            for &k in &[0, 2, 1, 3] {
                a = round3(a, b, c, d, x[k], 3);
                d = round3(d, a, b, c, x[k + 8], 9);
                c = round3(c, d, a, b, x[k + 4], 11);
                b = round3(b, c, d, a, x[k + 12], 15);
            }
            self.status = [
                self.status[0].wrapping_add(a),
                self.status[1].wrapping_add(b),
                self.status[2].wrapping_add(c),
                self.status[3].wrapping_add(d),
            ];
        }
        for i in 0..4 {
            self.status[i] = self.status[i].swap_bytes();
        }
    }
    fn hash(input: &[u8]) -> Vec<u8> {
        let mut md4 = Self::new();
        md4.input = input.to_vec();
        md4.padding();
        md4.round();
        md4.status[0..4]
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
