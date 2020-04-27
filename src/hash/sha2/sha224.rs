use super::Sha2;

use wasm_bindgen::prelude::*;

#[rustfmt::skip]
pub const H224: [u32; 8] = [
    0xC105_9ED8, 0x367C_D507, 0x3070_DD17, 0xF70_E5939,
    0xFFC0_0B31, 0x6858_1511, 0x64F9_8FA7, 0xBEF_A4FA4
];

#[wasm_bindgen]
pub struct Sha224(Sha2<u32>);

#[wasm_bindgen]
impl Sha224 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Sha2::<u32> {
            message: Vec::new(),
            word_block: Vec::new(),
            status: H224,
        })
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut sha224 = Self::new();
        sha224.0.message(message);
        sha224.0.padding();
        sha224.0.round();
        sha224.0.status[0..7]
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
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
