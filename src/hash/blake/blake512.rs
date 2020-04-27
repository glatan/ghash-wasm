use super::Blake;
use wasm_bindgen::prelude::*;

#[rustfmt::skip]
const H512: [u64; 8] = [
    0x6A09_E667_F3BC_C908, 0xBB67_AE85_84CA_A73B, 0x3C6E_F372_FE94_F82B, 0xA54F_F53A_5F1D_36F1,
    0x510E_527F_ADE6_82D1, 0x9B05_688C_2B3E_6C1F, 0x1F83_D9AB_FB41_BD6B, 0x5BE0_CD19_137E_2179
];

#[wasm_bindgen]
pub struct Blake512(Blake<u64>);

#[wasm_bindgen]
impl Blake512 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(Blake::<u64> {
            message: Vec::new(),
            word_block: Vec::new(),
            salt: [0; 4],
            l: 0,
            h: H512,
            t: [0; 2],
            v: [0; 16],
            bit: 512,
        })
    }
    fn hash_to_bytes(message: &[u8]) -> Vec<u8> {
        let mut blake512 = Self::new();
        blake512.0.message(message);
        blake512.0.set_counter();
        blake512.0.padding();
        blake512.0.compress();
        blake512
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
