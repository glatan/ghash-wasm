use super::Blake;
use wasm_bindgen::prelude::*;

#[rustfmt::skip]
const H224: [u32; 8] = [
    0xC105_9ED8, 0x367C_D507, 0x3070_DD17, 0xF70E_5939,
    0xFFC0_0B31, 0x6858_1511, 0x64F9_8FA7, 0xBEF_A4FA4
];

#[wasm_bindgen]
pub struct Blake224(Blake<u32>);

#[wasm_bindgen]
impl Blake224 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Blake::<u32> {
            message: Vec::new(),
            word_block: Vec::new(),
            salt: [0; 4],
            l: 0,
            h: H224,
            t: [0; 2],
            v: [0; 16],
            bit: 224,
        })
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut blake224 = Self::new();
        blake224.0.message(message);
        blake224.0.set_counter();
        blake224.0.padding();
        blake224.0.compress();
        blake224.0.h[0..7]
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
