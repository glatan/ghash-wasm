use super::Sha2_32bit;

use wasm_bindgen::prelude::*;

#[rustfmt::skip]
pub const H256: [u32; 8] = [
    0x6A09_E667, 0xBB67_AE85, 0x3C6E_F372, 0xA54F_F53A,
    0x510E_527F, 0x9B05_688C, 0x1F83_D9AB, 0x5BE0_CD19,
];

#[wasm_bindgen]
pub struct Sha256(Sha2_32bit);

#[wasm_bindgen]
impl Sha256 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Sha2_32bit {
            input: Vec::new(),
            word_block: Vec::new(),
            status: H256,
        })
    }
    fn padding(&mut self) {
        self.0.padding();
    }
    fn round(&mut self) {
        self.0.round();
    }
    fn hash(input: &[u8]) -> Vec<u8> {
        let mut sha256 = Self::new();
        sha256.0.input = input.to_vec();
        sha256.padding();
        sha256.round();
        sha256
            .0
            .status
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
