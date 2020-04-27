use super::Sha2;

use wasm_bindgen::prelude::*;

#[rustfmt::skip]
pub const H384: [u64; 8] = [
    0xCBBB_9D5D_C105_9ED8, 0x629A_292A_367C_D507, 0x9159_015A_3070_DD17, 0x152F_ECD8_F70E_5939,
    0x6733_2667_FFC0_0B31, 0x8EB4_4A87_6858_1511, 0xDB0C_2E0D_64F9_8FA7, 0x47B5_481D_BEFA_4FA4,
];

#[wasm_bindgen]
pub struct Sha384(Sha2<u64>);

#[wasm_bindgen]
impl Sha384 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Sha2::<u64> {
            message: Vec::new(),
            word_block: Vec::new(),
            status: H384,
        })
    }
    fn padding(&mut self) {
        self.0.padding();
    }
    fn round(&mut self) {
        self.0.round();
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut sha384 = Self::new();
        sha384.0.message(message);
        sha384.padding();
        sha384.round();
        sha384.0.status[0..6]
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
