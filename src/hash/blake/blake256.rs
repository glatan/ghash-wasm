use super::Blake;
use wasm_bindgen::prelude::*;

#[rustfmt::skip]
const H256: [u32; 8] = [
    0x6A09_E667, 0xBB67_AE85, 0x3C6E_F372, 0xA54F_F53A,
    0x510E_527F, 0x9B05_688C, 0x1F83_D9AB, 0x5BE0_CD19
];

#[wasm_bindgen]
pub struct Blake256(Blake<u32>);

#[wasm_bindgen]
impl Blake256 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Blake::<u32> {
            message: Vec::new(),
            word_block: Vec::new(),
            salt: [0; 4],
            l: 0,
            h: H256,
            t: [0; 2],
            v: [0; 16],
            bit: 256,
        })
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut blake256 = Self::new();
        blake256.0.message(message);
        blake256.0.set_counter();
        blake256.0.padding();
        blake256.0.compress();
        blake256
            .0
            .h
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
