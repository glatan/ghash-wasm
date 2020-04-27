use super::Sha2;

use wasm_bindgen::prelude::*;

#[rustfmt::skip]
pub const H512_TRUNC_224: [u64; 8] = [
    0x8C3D_37C8_1954_4DA2, 0x73E1_9966_89DC_D4D6, 0x1DFA_B7AE_32FF_9C82, 0x679D_D514_582F_9FCF,
    0x0F6D_2B69_7BD4_4DA8, 0x77E3_6F73_04C4_8942, 0x3F9D_85A8_6A1D_36C8, 0x1112_E6AD_91D6_92A1,
];

#[wasm_bindgen]
pub struct Sha512Trunc224(Sha2<u64>);

#[wasm_bindgen]
impl Sha512Trunc224 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Sha2::<u64> {
            message: Vec::new(),
            word_block: Vec::new(),
            status: H512_TRUNC_224,
        })
    }
    fn padding(&mut self) {
        self.0.padding();
    }
    fn round(&mut self) {
        self.0.round();
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut sha512trunc224 = Self::new();
        sha512trunc224.0.message(message);
        sha512trunc224.padding();
        sha512trunc224.round();
        sha512trunc224.0.status[0..4]
            .iter()
            .flat_map(|word| word.to_be_bytes().to_vec())
            .take(224 / 8) // (224 / 8) bytes
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
