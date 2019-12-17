use crate::shared::io::{Hash, New};
const WORD_BUFFER: [u32; 4] = [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];

struct Round;
impl Round {
    fn round1(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
        fn f(x: u32, y: u32, z: u32) -> u32 {
            (x & y) | (!x & z)
        }
        a.wrapping_add(f(b, c, d)).wrapping_add(k).rotate_left(s)
    }
    fn round2(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
        fn g(x: u32, y: u32, z: u32) -> u32 {
            (x & y) | (x & z) | (y & z)
        }
        a.wrapping_add(g(b, c, d))
            .wrapping_add(k)
            .wrapping_add(0x5A82_7999)
            .rotate_left(s)
    }
    fn round3(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
        fn h(x: u32, y: u32, z: u32) -> u32 {
            x ^ y ^ z
        }
        a.wrapping_add(h(b, c, d))
            .wrapping_add(k)
            .wrapping_add(0x6ED9_EBA1)
            .rotate_left(s)
    }
}

pub struct Md4Ctx {
    input_cache: Vec<u8>,
    word_block: Vec<u32>,
    status: [u32; 4],
}

impl Md4Ctx {
    fn padding(&mut self) {
        // word_block末尾に0x80を追加
        let input_length = self.input_cache.len();
        self.input_cache.push(0x80);
        let message_length: usize = self.input_cache.len();
        // (self.word_block.len() % 64)が56になるよう0を追加する数
        let padding_range = 56 - (message_length % 64);
        if padding_range != 0 {
            self.input_cache.append(&mut vec![0; padding_range]);
        } else {
            self.input_cache.append(&mut vec![0; 64]);
        }
        // 入力データの長さを追加
        let mut padding_length = { (8 * input_length).to_le_bytes().to_vec() };
        self.input_cache.append(&mut padding_length);
        // word_block用に値をu32に拡張する
        let mut word_block: Vec<u32> = Vec::new();
        // iは4の倍数となる (0, 4, 8..60..)
        for i in (0..self.input_cache.len()).filter(|i| i % 4 == 0) {
            word_block.push(u32::from_le_bytes([
                self.input_cache[i],
                self.input_cache[i + 1],
                self.input_cache[i + 2],
                self.input_cache[i + 3],
            ]));
        }
        self.word_block = word_block;
    }
    fn round(&mut self) {
        let word_block_length = self.word_block.len() / 16;
        let (mut a, mut b, mut c, mut d);
        let mut x: [u32; 16] = [0; 16];
        for i in 0..word_block_length {
            for j in 0..16 {
                x[j] = self.word_block[16 * i + j];
            }
            a = self.status[0];
            b = self.status[1];
            c = self.status[2];
            d = self.status[3];
            // Round 1
            for &k in &[0, 4, 8, 12] {
                a = Round::round1(a, b, c, d, x[k], 3);
                d = Round::round1(d, a, b, c, x[k + 1], 7);
                c = Round::round1(c, d, a, b, x[k + 2], 11);
                b = Round::round1(b, c, d, a, x[k + 3], 19);
            }
            // Round 2
            for k in 0..4 {
                a = Round::round2(a, b, c, d, x[k], 3);
                d = Round::round2(d, a, b, c, x[k + 4], 5);
                c = Round::round2(c, d, a, b, x[k + 8], 9);
                b = Round::round2(b, c, d, a, x[k + 12], 13);
            }
            // Round 3
            for &k in &[0, 2, 1, 3] {
                a = Round::round3(a, b, c, d, x[k], 3);
                d = Round::round3(d, a, b, c, x[k + 8], 9);
                c = Round::round3(c, d, a, b, x[k + 4], 11);
                b = Round::round3(b, c, d, a, x[k + 12], 15);
            }
            self.status = [
                self.status[0].wrapping_add(a),
                self.status[1].wrapping_add(b),
                self.status[2].wrapping_add(c),
                self.status[3].wrapping_add(d),
            ];
        }
        for i in 0..4 {
            self.status[i] = self.status[i].swap_bytes();
        }
    }
}

impl New for Md4Ctx {
    fn new(input: &[u8]) -> Md4Ctx {
        Md4Ctx {
            input_cache: input.to_vec(),
            word_block: Vec::new(),
            status: WORD_BUFFER,
        }
    }
}

impl Hash for Md4Ctx {
    fn hash(input: &[u8]) -> String {
        let mut md4ctx = Md4Ctx::new(&input);
        Md4Ctx::padding(&mut md4ctx);
        Md4Ctx::round(&mut md4ctx);

        md4ctx.status[0..4]
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}
