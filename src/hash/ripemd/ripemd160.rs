use super::Md4Padding;
use super::{f, K160_LEFT, K160_RIGHT, R_LEFT, R_RIGHT, S_LEFT, S_RIGHT};

use wasm_bindgen::prelude::*;

const H: [u32; 5] = [
    0x6745_2301,
    0xEFCD_AB89,
    0x98BA_DCFE,
    0x1032_5476,
    0xC3D2_E1F0,
];

#[wasm_bindgen]
pub struct Ripemd160 {
    input: Vec<u8>,
    word_block: Vec<u32>,
    status: [u32; 5],
}

#[wasm_bindgen]
impl Ripemd160 {
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
            let [mut a_left, mut b_left, mut c_left, mut d_left, mut e_left] = self.status;
            let [mut a_right, mut b_right, mut c_right, mut d_right, mut e_right] = self.status;
            for j in 0..80 {
                t = a_left
                    .wrapping_add(f(j, b_left, c_left, d_left))
                    .wrapping_add(self.word_block[i * 16 + R_LEFT[j]])
                    .wrapping_add(K160_LEFT[(j / 16)])
                    .rotate_left(S_LEFT[j])
                    .wrapping_add(e_left);
                a_left = e_left;
                e_left = d_left;
                d_left = c_left.rotate_left(10);
                c_left = b_left;
                b_left = t;
                t = a_right
                    .wrapping_add(f(79 - j, b_right, c_right, d_right))
                    .wrapping_add(self.word_block[i * 16 + R_RIGHT[j]])
                    .wrapping_add(K160_RIGHT[(j / 16)])
                    .rotate_left(S_RIGHT[j])
                    .wrapping_add(e_right);
                a_right = e_right;
                e_right = d_right;
                d_right = c_right.rotate_left(10);
                c_right = b_right;
                b_right = t;
            }
            t = self.status[1].wrapping_add(c_left).wrapping_add(d_right);
            self.status[1] = self.status[2].wrapping_add(d_left).wrapping_add(e_right);
            self.status[2] = self.status[3].wrapping_add(e_left).wrapping_add(a_right);
            self.status[3] = self.status[4].wrapping_add(a_left).wrapping_add(b_right);
            self.status[4] = self.status[0].wrapping_add(b_left).wrapping_add(c_right);
            self.status[0] = t;
        }
    }
    fn hash(input: &[u8]) -> Vec<u8> {
        let mut ripemd160 = Self::new();
        ripemd160.input = input.to_vec();
        ripemd160.padding();
        ripemd160.round();
        ripemd160
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

impl Md4Padding for Ripemd160 {
    fn u64_to_bytes(num: u64) -> [u8; 8] {
        num.to_le_bytes()
    }
    fn u32_from_bytes(bytes: [u8; 4]) -> u32 {
        u32::from_le_bytes(bytes)
    }
}
