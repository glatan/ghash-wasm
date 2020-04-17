use std::cmp::Ordering;

mod sha224;
mod sha256;
mod sha384;
mod sha512;
mod sha512trunc224;
mod sha512trunc256;

pub use sha224::Sha224;
pub use sha256::Sha256;
pub use sha384::Sha384;
pub use sha512::Sha512;
pub use sha512trunc224::Sha512Trunc224;
pub use sha512trunc256::Sha512Trunc256;

// SHA-224 and SHA-256 Constant
#[rustfmt::skip]
const K32: [u32; 64] = [
    0x428A_2F98, 0x7137_4491, 0xB5C0_FBCF, 0xE9B5_DBA5, 0x3956_C25B, 0x59F1_11F1, 0x923F_82A4, 0xAB1_C5ED5,
    0xD807_AA98, 0x1283_5B01, 0x2431_85BE, 0x550C_7DC3, 0x72BE_5D74, 0x80DE_B1FE, 0x9BDC_06A7, 0xC19_BF174,
    0xE49B_69C1, 0xEFBE_4786, 0x0FC1_9DC6, 0x240C_A1CC, 0x2DE9_2C6F, 0x4A74_84AA, 0x5CB0_A9DC, 0x76F_988DA,
    0x983E_5152, 0xA831_C66D, 0xB003_27C8, 0xBF59_7FC7, 0xC6E0_0BF3, 0xD5A7_9147, 0x06CA_6351, 0x142_92967,
    0x27B7_0A85, 0x2E1B_2138, 0x4D2C_6DFC, 0x5338_0D13, 0x650A_7354, 0x766A_0ABB, 0x81C2_C92E, 0x927_22C85,
    0xA2BF_E8A1, 0xA81A_664B, 0xC24B_8B70, 0xC76C_51A3, 0xD192_E819, 0xD699_0624, 0xF40E_3585, 0x106_AA070,
    0x19A4_C116, 0x1E37_6C08, 0x2748_774C, 0x34B0_BCB5, 0x391C_0CB3, 0x4ED8_AA4A, 0x5B9C_CA4F, 0x682_E6FF3,
    0x748F_82EE, 0x78A5_636F, 0x84C8_7814, 0x8CC7_0208, 0x90BE_FFFA, 0xA450_6CEB, 0xBEF9_A3F7, 0xC67_178F2,
];
// SHA-384, SHA-512, SHA-512/224 and SHA-512/256 Constant
#[rustfmt::skip]
const K64: [u64; 80] = [
    0x428A_2F98_D728_AE22, 0x7137_4491_23EF_65CD, 0xB5C0_FBCF_EC4D_3B2F, 0xE9B5_DBA5_8189_DBBC,
    0x3956_C25B_F348_B538, 0x59F1_11F1_B605_D019, 0x923F_82A4_AF19_4F9B, 0xAB1C_5ED5_DA6D_8118,
    0xD807_AA98_A303_0242, 0x1283_5B01_4570_6FBE, 0x2431_85BE_4EE4_B28C, 0x550C_7DC3_D5FF_B4E2,
    0x72BE_5D74_F27B_896F, 0x80DE_B1FE_3B16_96B1, 0x9BDC_06A7_25C7_1235, 0xC19B_F174_CF69_2694,
    0xE49B_69C1_9EF1_4AD2, 0xEFBE_4786_384F_25E3, 0x0FC1_9DC6_8B8C_D5B5, 0x240C_A1CC_77AC_9C65,
    0x2DE9_2C6F_592B_0275, 0x4A74_84AA_6EA6_E483, 0x5CB0_A9DC_BD41_FBD4, 0x76F9_88DA_8311_53B5,
    0x983E_5152_EE66_DFAB, 0xA831_C66D_2DB4_3210, 0xB003_27C8_98FB_213F, 0xBF59_7FC7_BEEF_0EE4,
    0xC6E0_0BF3_3DA8_8FC2, 0xD5A7_9147_930A_A725, 0x06CA_6351_E003_826F, 0x1429_2967_0A0E_6E70,
    0x27B7_0A85_46D2_2FFC, 0x2E1B_2138_5C26_C926, 0x4D2C_6DFC_5AC4_2AED, 0x5338_0D13_9D95_B3DF,
    0x650A_7354_8BAF_63DE, 0x766A_0ABB_3C77_B2A8, 0x81C2_C92E_47ED_AEE6, 0x9272_2C85_1482_353B,
    0xA2BF_E8A1_4CF1_0364, 0xA81A_664B_BC42_3001, 0xC24B_8B70_D0F8_9791, 0xC76C_51A3_0654_BE30,
    0xD192_E819_D6EF_5218, 0xD699_0624_5565_A910, 0xF40E_3585_5771_202A, 0x106A_A070_32BB_D1B8,
    0x19A4_C116_B8D2_D0C8, 0x1E37_6C08_5141_AB53, 0x2748_774C_DF8E_EB99, 0x34B0_BCB5_E19B_48A8,
    0x391C_0CB3_C5C9_5A63, 0x4ED8_AA4A_E341_8ACB, 0x5B9C_CA4F_7763_E373, 0x682E_6FF3_D6B2_B8A3,
    0x748F_82EE_5DEF_B2FC, 0x78A5_636F_4317_2F60, 0x84C8_7814_A1F0_AB72, 0x8CC7_0208_1A64_39EC,
    0x90BE_FFFA_2363_1E28, 0xA450_6CEB_DE82_BDE9, 0xBEF9_A3F7_B2C6_7915, 0xC671_78F2_E372_532B,
    0xCA27_3ECE_EA26_619C, 0xD186_B8C7_21C0_C207, 0xEADA_7DD6_CDE0_EB1E, 0xF57D_4F7F_EE6E_D178,
    0x06F0_67AA_7217_6FBA, 0x0A63_7DC5_A2C8_98A6, 0x113F_9804_BEF9_0DAE, 0x1B71_0B35_131C_471B,
    0x28DB_77F5_2304_7D84, 0x32CA_AB7B_40C7_2493, 0x3C9E_BE0A_15C9_BEBC, 0x431D_67C4_9C10_0D4C,
    0x4CC5_D4BE_CB3E_42B6, 0x597F_299C_FC65_7E2A, 0x5FCB_6FAB_3AD6_FAEC, 0x6C44_198C_4A47_5817,
];

// SHA-224 and SHA-256 Functions
const fn ch32(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}
const fn maj32(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}
const fn big_sigma32_0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}
const fn big_sigma32_1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}
const fn small_sigma32_0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}
const fn small_sigma32_1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

// SHA-384, SHA-512, SHA-512/224 and SHA-512/256 Functions
const fn ch64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!x & z)
}
const fn maj64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}
const fn big_sigma64_0(x: u64) -> u64 {
    x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
}
const fn big_sigma64_1(x: u64) -> u64 {
    x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
}
const fn small_sigma64_0(x: u64) -> u64 {
    x.rotate_right(1) ^ x.rotate_right(8) ^ (x >> 7)
}
const fn small_sigma64_1(x: u64) -> u64 {
    x.rotate_right(19) ^ x.rotate_right(61) ^ (x >> 6)
}

struct Sha2_32bit {
    input: Vec<u8>,
    word_block: Vec<u32>,
    status: [u32; 8],
}

struct Sha2_64bit {
    input: Vec<u8>,
    word_block: Vec<u64>,
    status: [u64; 8],
}

impl Sha2_32bit {
    fn padding(&mut self) {
        let input_length = self.input.len();
        // word_block末尾に0x80を追加(0b1000_0000)
        self.input.push(0x80);
        // [byte]: 64 - 8(input_length) - 1(0x80) = 55
        let padding_length = 55 - (input_length as i128);
        match padding_length.cmp(&0) {
            Ordering::Greater => {
                self.input.append(&mut vec![0; padding_length as usize]);
            }
            Ordering::Less => {
                self.input
                    .append(&mut vec![0; 64 - (padding_length.abs() % 64) as usize]);
            }
            Ordering::Equal => (),
        }
        // 入力データの長さを追加
        self.input
            .append(&mut (8 * input_length as u64).to_be_bytes().to_vec());
        // iは4の倍数となる (0, 4, 8..60..)
        for i in (0..self.input.len()).filter(|i| i % 4 == 0) {
            self.word_block.push(u32::from_be_bytes([
                self.input[i],
                self.input[i + 1],
                self.input[i + 2],
                self.input[i + 3],
            ]));
        }
    }
    #[allow(clippy::many_single_char_names)]
    fn round(&mut self) {
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h);
        let (mut temp_1, mut temp_2);
        let mut w = [0; 64];
        for i in 0..(self.word_block.len() / 16) {
            for t in 0..16 {
                w[t] = self.word_block[t + i * 16];
            }
            for t in 16..64 {
                w[t] = small_sigma32_1(w[t - 2])
                    .wrapping_add(w[t - 7])
                    .wrapping_add(small_sigma32_0(w[t - 15]))
                    .wrapping_add(w[t - 16]);
            }
            a = self.status[0];
            b = self.status[1];
            c = self.status[2];
            d = self.status[3];
            e = self.status[4];
            f = self.status[5];
            g = self.status[6];
            h = self.status[7];
            for t in 0..64 {
                temp_1 = h
                    .wrapping_add(big_sigma32_1(e))
                    .wrapping_add(ch32(e, f, g))
                    .wrapping_add(K32[t])
                    .wrapping_add(w[t]);
                temp_2 = big_sigma32_0(a).wrapping_add(maj32(a, b, c));
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp_1);
                d = c;
                c = b;
                b = a;
                a = temp_1.wrapping_add(temp_2);
            }
            self.status[0] = self.status[0].wrapping_add(a);
            self.status[1] = self.status[1].wrapping_add(b);
            self.status[2] = self.status[2].wrapping_add(c);
            self.status[3] = self.status[3].wrapping_add(d);
            self.status[4] = self.status[4].wrapping_add(e);
            self.status[5] = self.status[5].wrapping_add(f);
            self.status[6] = self.status[6].wrapping_add(g);
            self.status[7] = self.status[7].wrapping_add(h);
        }
    }
}

impl Sha2_64bit {
    fn padding(&mut self) {
        let input_length = self.input.len();
        // word_block末尾に0x80を追加(0b1000_0000)
        self.input.push(0x80);
        // [byte]: 128 - 16(input_length) - 1(0x80) = 111
        let padding_length = 111 - (input_length as i128);
        match padding_length.cmp(&0) {
            Ordering::Greater => {
                self.input.append(&mut vec![0; padding_length as usize]);
            }
            Ordering::Less => {
                self.input
                    .append(&mut vec![0; 128 - (padding_length.abs() % 128) as usize]);
            }
            Ordering::Equal => (),
        }
        // 入力データの長さを追加
        self.input
            .append(&mut (8 * input_length as u128).to_be_bytes().to_vec());
        // 64bitワードにしてpush
        for i in (0..self.input.len()).filter(|i| i % 8 == 0) {
            self.word_block.push(u64::from_be_bytes([
                self.input[i],
                self.input[i + 1],
                self.input[i + 2],
                self.input[i + 3],
                self.input[i + 4],
                self.input[i + 5],
                self.input[i + 6],
                self.input[i + 7],
            ]));
        }
    }
    #[allow(clippy::many_single_char_names)]
    fn round(&mut self) {
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h);
        let (mut temp_1, mut temp_2);
        let mut w = [0; 80];
        for i in 0..(self.word_block.len() / 16) {
            for t in 0..16 {
                w[t] = self.word_block[t + i * 16];
            }
            for t in 16..80 {
                w[t] = small_sigma64_1(w[t - 2])
                    .wrapping_add(w[t - 7])
                    .wrapping_add(small_sigma64_0(w[t - 15]))
                    .wrapping_add(w[t - 16]);
            }
            a = self.status[0];
            b = self.status[1];
            c = self.status[2];
            d = self.status[3];
            e = self.status[4];
            f = self.status[5];
            g = self.status[6];
            h = self.status[7];
            for t in 0..80 {
                temp_1 = h
                    .wrapping_add(big_sigma64_1(e))
                    .wrapping_add(ch64(e, f, g))
                    .wrapping_add(K64[t])
                    .wrapping_add(w[t]);
                temp_2 = big_sigma64_0(a).wrapping_add(maj64(a, b, c));
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp_1);
                d = c;
                c = b;
                b = a;
                a = temp_1.wrapping_add(temp_2);
            }
            self.status[0] = self.status[0].wrapping_add(a);
            self.status[1] = self.status[1].wrapping_add(b);
            self.status[2] = self.status[2].wrapping_add(c);
            self.status[3] = self.status[3].wrapping_add(d);
            self.status[4] = self.status[4].wrapping_add(e);
            self.status[5] = self.status[5].wrapping_add(f);
            self.status[6] = self.status[6].wrapping_add(g);
            self.status[7] = self.status[7].wrapping_add(h);
        }
    }
}
