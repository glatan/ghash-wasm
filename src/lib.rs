mod md2;
mod md4;
mod md5;
mod sha0;
mod sha1;

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

pub use self::{
    md2::Md2Ctx as Md2,
    md4::Md4Ctx as Md4,
    md5::Md5Ctx as Md5,
    sha0::Sha0,
    sha1::Sha1,
};
