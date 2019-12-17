use ghash::shared::io::Hash;
use ghash::{Md2, Md4, Md5};
use std::io;
use std::io::BufRead;

fn main() {
    let input: Vec<u8> = {
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim_end().as_bytes().to_owned()
    };
    println!("input: {:?}", input);
    let md2_result: String = Md2::hash(&input);
    let md4_result: String = Md4::hash(&input);
    let md5_result: String = Md5::hash(&input);

    println!("MD2: {:}", md2_result);
    println!("MD4: {:}", md4_result);
    println!("MD5: {:}", md5_result);
}
