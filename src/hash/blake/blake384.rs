use super::Blake;
use wasm_bindgen::prelude::*;

#[rustfmt::skip]
const H384: [u64; 8] = [
    0xCBBB_9D5D_C105_9ED8, 0x629A_292A_367C_D507, 0x9159_015A_3070_DD17, 0x152F_ECD8_F70E_5939,
    0x6733_2667_FFC0_0B31, 0x8EB4_4A87_6858_1511, 0xDB0C_2E0D_64F9_8FA7, 0x47B5_481D_BEFA_4FA4
];

#[wasm_bindgen]
pub struct Blake384(Blake<u64>);

#[wasm_bindgen]
impl Blake384 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Blake::<u64> {
            message: Vec::new(),
            word_block: Vec::new(),
            salt: [0; 4],
            l: 0,
            h: H384,
            t: [0; 2],
            v: [0; 16],
            bit: 384,
        })
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut blake384 = Self::new();
        blake384.0.message(message);
        blake384.0.set_counter();
        blake384.0.padding();
        blake384.0.compress();
        blake384.0.h[0..6]
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
