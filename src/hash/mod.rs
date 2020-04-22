mod md2;
mod md4;
mod md5;
mod ripemd;
mod sha0;
mod sha1;
mod sha2;

use md4::Md4Padding;

pub use md2::Md2;
pub use md4::Md4;
pub use md5::Md5;
pub use ripemd::{Ripemd128, Ripemd160, Ripemd256, Ripemd320};
pub use sha0::Sha0;
pub use sha1::Sha1;
pub use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};

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
