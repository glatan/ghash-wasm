use super::{f, K160_LEFT, K160_RIGHT, R_LEFT, R_RIGHT, S_LEFT, S_RIGHT};
use crate::{impl_md4_padding, impl_message};
use std::cmp::Ordering;
use std::mem;

use wasm_bindgen::prelude::*;

#[rustfmt::skip]
const H: [u32; 10] = [
    0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476, 0xC3D2_E1F0,
    0x7654_3210, 0xFEDC_BA98, 0x89AB_CDEF, 0x0123_4567, 0x3C2D_1E0F
];

#[wasm_bindgen]
pub struct Ripemd320 {
    message: Vec<u8>,
    word_block: Vec<u32>,
    status: [u32; 10],
}

#[wasm_bindgen]
impl Ripemd320 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            message: Vec::new(),
            word_block: Vec::new(),
            status: H,
        }
    }
    fn round(&mut self) {
        let mut t;
        for i in 0..(self.word_block.len() / 16) {
            let (mut a_left, mut b_left, mut c_left, mut d_left, mut e_left) = (
                self.status[0],
                self.status[1],
                self.status[2],
                self.status[3],
                self.status[4],
            );
            let (mut a_right, mut b_right, mut c_right, mut d_right, mut e_right) = (
                self.status[5],
                self.status[6],
                self.status[7],
                self.status[8],
                self.status[9],
            );
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
                if j == 15 {
                    t = b_left;
                    b_left = b_right;
                    b_right = t;
                } else if j == 31 {
                    t = d_left;
                    d_left = d_right;
                    d_right = t;
                } else if j == 47 {
                    t = a_left;
                    a_left = a_right;
                    a_right = t;
                } else if j == 63 {
                    t = c_left;
                    c_left = c_right;
                    c_right = t;
                } else if j == 79 {
                    t = e_left;
                    e_left = e_right;
                    e_right = t;
                }
            }
            self.status[0] = self.status[0].wrapping_add(a_left);
            self.status[1] = self.status[1].wrapping_add(b_left);
            self.status[2] = self.status[2].wrapping_add(c_left);
            self.status[3] = self.status[3].wrapping_add(d_left);
            self.status[4] = self.status[4].wrapping_add(e_left);
            self.status[5] = self.status[5].wrapping_add(a_right);
            self.status[6] = self.status[6].wrapping_add(b_right);
            self.status[7] = self.status[7].wrapping_add(c_right);
            self.status[8] = self.status[8].wrapping_add(d_right);
            self.status[9] = self.status[9].wrapping_add(e_right);
        }
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut ripemd320 = Self::new();
        ripemd320.message(message);
        ripemd320.padding();
        ripemd320.round();
        ripemd320
            .status
            .iter()
            .flat_map(|word| word.to_le_bytes().to_vec())
            .collect()
    }
    #[wasm_bindgen]
    pub fn hash_to_lowercase(message: &[u8]) -> String {
        Self::hash_to_bytes(message)
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}

impl Ripemd320 {
    impl_message!(self, u64);
    impl_md4_padding!(u32 => self, from_le_bytes, to_le_bytes, 55, {});
}
