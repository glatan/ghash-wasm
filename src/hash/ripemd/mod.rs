mod ripemd128;
mod ripemd160;
mod ripemd256;
mod ripemd320;

pub use ripemd128::Ripemd128;
pub use ripemd160::Ripemd160;
pub use ripemd256::Ripemd256;
pub use ripemd320::Ripemd320;

// RIPEMD-128 and RIPEMD-256 Constants
const K128_LEFT: [u32; 4] = [
    0x0000_0000, // (0 <= j <= 15)
    0x5A82_7999, // (16 <= j <= 31) int(2**30 x sqrt(2))
    0x6ED9_EBA1, // (32 <= j <= 47) int(2**30 x sqrt(3))
    0x8F1B_BCDC, // (48 <= j <= 63) int(2**30 x sqrt(5))
];
const K128_RIGHT: [u32; 4] = [
    0x50A2_8BE6, // (0 <= j <= 15) int(2**30 x cbrt(2))
    0x5C4D_D124, // (16 <= j <= 31) int(2**30 x cbrt(3))
    0x6D70_3EF3, // (32 <= j <= 47) int(2**30 x cbrt(5))
    0x0000_0000, // (47 <= j <= 63)
];
// RIPEMD-160 and RIPEMD-320 Constants
const K160_LEFT: [u32; 5] = [
    0x0000_0000, // (0 <= j <= 15)
    0x5A82_7999, // (16 <= j <= 31) int(2**30 x sqrt(2))
    0x6ED9_EBA1, // (32 <= j <= 47) int(2**30 x sqrt(3))
    0x8F1B_BCDC, // (48 <= j <= 63) int(2**30 x sqrt(5))
    0xA953_FD4E, // (64 <= j <= 79) int(2**30 x sqrt(7))
];
const K160_RIGHT: [u32; 5] = [
    0x50A2_8BE6, // (0 <= j <= 15) int(2**30 x cbrt(2))
    0x5C4D_D124, // (16 <= j <= 31) int(2**30 x cbrt(3))
    0x6D70_3EF3, // (32 <= j <= 47) int(2**30 x cbrt(5))
    0x7A6D_76E9, // (48 <= j <= 63) int(2**30 x cbrt(7))
    0x0000_0000, // (64 <= j <= 79)
];
// All RIPEMD Constants
#[rustfmt::skip]
const R_LEFT: [usize; 80] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // 0 <= j <= 15
    7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5, 2, 14, 11, 8, // 16 <= j <= 31
    3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12, // 32 <= j <= 47
    1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7, 15, 14, 5, 6, 2, // 48 <= j <= 63
    4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13, // 64 <= j <= 64
];
#[rustfmt::skip]
const R_RIGHT: [usize; 80] = [
    5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12, // 0 <= j <= 15
    6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12, 4, 9, 1, 2, // 16 <= j <= 31
    15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13, // 32 <= j <= 47
    8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2, 13, 9, 7, 10, 14, // 48 <= j <= 63
    12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11, // 64 <= j <= 64
];
#[rustfmt::skip]
const S_LEFT: [u32; 80] = [
    11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8, // 0 <= j <= 15
    7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15, 9, 11, 7, 13, 12, // 16 <= j <= 31
    11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5, // 32 <= j <= 47
    11, 12, 14, 15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12, // 48 <= j <= 63
    9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6, // 64 <= j <= 64
];
#[rustfmt::skip]
const S_RIGHT: [u32; 80] = [
    8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6, // 0 <= j <= 15
    9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12, 7, 6, 15, 13, 11, // 16 <= j <= 31
    9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5, // 32 <= j <= 47
    15, 5, 8, 11, 14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8, // 48 <= j <= 63
    8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11, // 64 <= j <= 64
];
// All RIPEMD Function
fn f(j: usize, x: u32, y: u32, z: u32) -> u32 {
    if j < 16 {
        return x ^ y ^ z;
    }
    if j >= 16 && j < 32 {
        return (x & y) | (!x & z);
    }
    if j >= 32 && j < 48 {
        return (x | !y) ^ z;
    }
    if j >= 48 && j < 64 {
        return (x & z) | (y & !z);
    }
    if j >= 64 && j < 80 {
        x ^ (y | !z)
    } else {
        panic!("j={}: j is must be between 0 and 79!", j);
    }
}
