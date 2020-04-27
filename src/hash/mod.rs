mod blake;
mod md2;
mod md4;
mod md5;
mod ripemd;
mod sha0;
mod sha1;
mod sha2;

pub use blake::{Blake224, Blake256, Blake384, Blake512};
pub use md2::Md2;
pub use md4::Md4;
pub use md5::Md5;
pub use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
pub use sha0::Sha0;
pub use sha1::Sha1;
pub use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};

#[macro_export(local)]
macro_rules! impl_message {
    ($self:ident, $LimitT:ty) => {
            fn message(&mut $self, message: &[u8]) {
                match message.len().checked_mul(8) {
                    Some(_) => {
                        // input bit length is less than usize::MAX
                        match mem::size_of::<usize>().cmp(&mem::size_of::<$LimitT>()) {
                            Ordering::Equal | Ordering::Less => {
                                // input type limit is less than hash function limit
                                $self.message = message.to_vec();
                            }
                            Ordering::Greater => {
                                // input bit length is greater than the hash function limit length
                                panic!(
                                    "{} takes a input of any length less than 2^{} bits",
                                    stringify!($SelfT),
                                    mem::size_of::<$LimitT>()
                                )
                            }
                        }
                    }
                    None => panic!(
                        "{} * 8 is greeter than usize::MAX",
                        mem::size_of::<$LimitT>()
                    ),
                }
            }
    };
}

// MD4 Style Padding
#[macro_export(local)]
macro_rules! impl_md4_padding {
    (u32 => $self:ident, $from_bytes:ident, $to_bytes:ident, $padding_base:expr, $optional_padding:block) => {
        fn padding(&mut $self) {
            let message_length = $self.message.len();
            // 入力末尾に0x80を追加(0b1000_0000)
            $self.message.push(0x80);
            // [byte]: 64 - 8(message_length) - 1(0x80) - 1(0x00 or 0x01) = 54 => BLAKE
            // [byte]: 64 - 8(message_length) - 1(0x80) = 55 => Others
            let padding_length = $padding_base - (message_length as i128);
            match padding_length.cmp(&0) {
                Ordering::Greater => {
                    $self.message.append(&mut vec![0; padding_length as usize]);
                }
                Ordering::Less => {
                    $self.message
                        .append(&mut vec![0; 64 - (padding_length.abs() % 64) as usize]);
                }
                Ordering::Equal => (),
            }
            // for BLAKE padding
            // BLAKE-224 => push 0x00
            // BLAKE-256 => push 0x01
            $optional_padding
            // 入力データの長さを追加
            $self.message
                .append(&mut (8 * message_length as u64).$to_bytes().to_vec());
            // バイト列からワードブロックを生成
            for i in (0..$self.message.len()).filter(|i| i % 4 == 0) {
                $self.word_block.push(u32::$from_bytes([
                    $self.message[i],
                    $self.message[i + 1],
                    $self.message[i + 2],
                    $self.message[i + 3],
                ]));
            }
        }
    };
    (u64 => $self:ident, $from_bytes:ident, $to_bytes:ident, $padding_base:expr, $optional_padding:block) => {
        fn padding(&mut $self) {
            let input_length = $self.message.len();
            // word_block末尾に0x80を追加(0b1000_0000)
            $self.message.push(0x80);
            // [byte]: 128 - 16(input_length) - 1(0x80) - 1(0x00 or 0x00)= 110 => BLAKE
            // [byte]: 128 - 16(input_length) - 1(0x80) = 111 => Others
            let padding_length = $padding_base - (input_length as i128);
            match padding_length.cmp(&0) {
                Ordering::Greater => {
                    $self.message.append(&mut vec![0; padding_length as usize]);
                }
                Ordering::Less => {
                    $self.message
                        .append(&mut vec![0; 128 - (padding_length.abs() % 128) as usize]);
                }
                Ordering::Equal => (),
            }
            // for BLAKE padding
            // BLAKE-384 => push 0x00
            // BLAKE-512 => push 0x01
            $optional_padding
            // 入力データの長さを追加
            $self.message
                .append(&mut (8 * input_length as u128).$to_bytes().to_vec());
            // 64bitワードにしてpush
            for i in (0..$self.message.len()).filter(|i| i % 8 == 0) {
                $self.word_block.push(u64::$from_bytes([
                    $self.message[i],
                    $self.message[i + 1],
                    $self.message[i + 2],
                    $self.message[i + 3],
                    $self.message[i + 4],
                    $self.message[i + 5],
                    $self.message[i + 6],
                    $self.message[i + 7],
                ]));
            }
        }
    };
}

#[cfg(debug_assertions)]
use wasm_bindgen::prelude::*;

#[cfg(debug_assertions)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
