use super::Sha2_64bit;

use wasm_bindgen::prelude::*;

#[rustfmt::skip]
pub const H512_TRUNC_256: [u64; 8] = [
    0x2231_2194_FC2B_F72C, 0x9F55_5FA3_C84C_64C2, 0x2393_B86B_6F53_B151, 0x9638_7719_5940_EABD,
    0x9628_3EE2_A88E_FFE3, 0xBE5E_1E25_5386_3992, 0x2B01_99FC_2C85_B8AA, 0x0EB7_2DDC_81C5_2CA2,
];

#[wasm_bindgen]
pub struct Sha512Trunc256(Sha2_64bit);

#[wasm_bindgen]
impl Sha512Trunc256 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Sha2_64bit {
            input: Vec::new(),
            word_block: Vec::new(),
            status: H512_TRUNC_256,
        })
    }
    fn padding(&mut self) {
        self.0.padding();
    }
    fn round(&mut self) {
        self.0.round();
    }
    fn hash(input: &[u8]) -> Vec<u8> {
        let mut sha512trunc256 = Self::new();
        sha512trunc256.0.input = input.to_vec();
        sha512trunc256.padding();
        sha512trunc256.round();
        sha512trunc256.0.status[0..4]
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
