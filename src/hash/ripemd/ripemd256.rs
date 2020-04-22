use super::Md4Padding;
use super::{f, K128_LEFT, K128_RIGHT, R_LEFT, R_RIGHT, S_LEFT, S_RIGHT};

use wasm_bindgen::prelude::*;

#[rustfmt::skip]
const H: [u32; 8] = [
    0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476,
    0x7654_3210, 0xFEDC_BA98, 0x89AB_CDEF, 0x0123_4567
];

#[wasm_bindgen]
pub struct Ripemd256 {
    input: Vec<u8>,
    word_block: Vec<u32>,
    status: [u32; 8],
}

#[wasm_bindgen]
impl Ripemd256 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            word_block: Vec::new(),
            status: H,
        }
    }
    fn padding(&mut self) {
        self.word_block = Self::md4_padding(&mut self.input);
    }
    fn round(&mut self) {
        let mut t;
        for i in 0..(self.word_block.len() / 16) {
            let [mut a_left, mut b_left, mut c_left, mut d_left] = [
                self.status[0],
                self.status[1],
                self.status[2],
                self.status[3],
            ];
            let [mut a_right, mut b_right, mut c_right, mut d_right] = [
                self.status[4],
                self.status[5],
                self.status[6],
                self.status[7],
            ];
            for j in 0..64 {
                t = a_left
                    .wrapping_add(f(j, b_left, c_left, d_left))
                    .wrapping_add(self.word_block[i * 16 + R_LEFT[j]])
                    .wrapping_add(K128_LEFT[(j / 16)])
                    .rotate_left(S_LEFT[j]);
                a_left = d_left;
                d_left = c_left;
                c_left = b_left;
                b_left = t;
                t = a_right
                    .wrapping_add(f(63 - j, b_right, c_right, d_right))
                    .wrapping_add(self.word_block[i * 16 + R_RIGHT[j]])
                    .wrapping_add(K128_RIGHT[(j / 16)])
                    .rotate_left(S_RIGHT[j]);
                a_right = d_right;
                d_right = c_right;
                c_right = b_right;
                b_right = t;
                if j == 15 {
                    t = a_left;
                    a_left = a_right;
                    a_right = t;
                } else if j == 31 {
                    t = b_left;
                    b_left = b_right;
                    b_right = t;
                } else if j == 47 {
                    t = c_left;
                    c_left = c_right;
                    c_right = t;
                } else if j == 63 {
                    t = d_left;
                    d_left = d_right;
                    d_right = t;
                }
            }
            self.status[0] = self.status[0].wrapping_add(a_left);
            self.status[1] = self.status[1].wrapping_add(b_left);
            self.status[2] = self.status[2].wrapping_add(c_left);
            self.status[3] = self.status[3].wrapping_add(d_left);
            self.status[4] = self.status[4].wrapping_add(a_right);
            self.status[5] = self.status[5].wrapping_add(b_right);
            self.status[6] = self.status[6].wrapping_add(c_right);
            self.status[7] = self.status[7].wrapping_add(d_right);
        }
    }
    fn hash(input: &[u8]) -> Vec<u8> {
        let mut ripemd256 = Self::new();
        ripemd256.input = input.to_vec();
        ripemd256.padding();
        ripemd256.round();
        ripemd256
            .status
            .iter()
            .flat_map(|word| word.to_le_bytes().to_vec())
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

impl Md4Padding for Ripemd256 {
    fn u64_to_bytes(num: u64) -> [u8; 8] {
        num.to_le_bytes()
    }
    fn u32_from_bytes(bytes: [u8; 4]) -> u32 {
        u32::from_le_bytes(bytes)
    }
}
