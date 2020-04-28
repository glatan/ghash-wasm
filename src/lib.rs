mod hash;

pub use hash::{
    Blake224, Blake256, Blake384, Blake512, Md2, Md4, Md5, Sha0, Sha1, Sha224, Sha256, Sha384,
    Sha512, Sha512Trunc224, Sha512Trunc256,
};

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use super::hash::*;
    #[wasm_bindgen_test]
    fn empty_input() {
        // BLAKE
        assert_eq!(Blake224::hash_to_lowercase(&[]), "7dc5313b1c04512a174bd6503b89607aecbee0903d40a8a569c94eed");
        assert_eq!(Blake256::hash_to_lowercase(&[]), "716f6e863f744b9ac22c97ec7b76ea5f5908bc5b2f67c61510bfc4751384ea7a");
        assert_eq!(Blake384::hash_to_lowercase(&[]), "c6cbd89c926ab525c242e6621f2f5fa73aa4afe3d9e24aed727faaadd6af38b620bdb623dd2b4788b1c8086984af8706");
        assert_eq!(Blake512::hash_to_lowercase(&[]), "a8cfbbd73726062df0c6864dda65defe58ef0cc52a5625090fa17601e1eecd1b628e94f396ae402a00acc9eab77b4d4c2e852aaaa25a636d80af3fc7913ef5b8");
        // MD2
        assert_eq!(Md2::hash_to_lowercase(&[]), "8350e5a3e24c153df2275c9f80692773");
        // MD4
        assert_eq!(Md4::hash_to_lowercase(&[]), "31d6cfe0d16ae931b73c59d7e0c089c0");
        // MD5
        assert_eq!(Md5::hash_to_lowercase(&[]), "d41d8cd98f00b204e9800998ecf8427e");
        // RIPEMD
        assert_eq!(Ripemd128::hash_to_lowercase(&[]), "cdf26213a150dc3ecb610f18f6b38b46");
        assert_eq!(Ripemd160::hash_to_lowercase(&[]), "9c1185a5c5e9fc54612808977ee8f548b2258d31");
        assert_eq!(Ripemd256::hash_to_lowercase(&[]), "02ba4c4e5f8ecd1877fc52d64d30e37a2d9774fb1e5d026380ae0168e3c5522d");
        assert_eq!(Ripemd320::hash_to_lowercase(&[]), "22d65d5661536cdc75c1fdf5c6de7b41b9f27325ebc61e8557177d705a0ec880151c3a32a00899b8");
        // SHA-0
        assert_eq!(Sha0::hash_to_lowercase(&[]), "f96cea198ad1dd5617ac084a3d92c6107708c0ef");
        // SHA-1
        assert_eq!(Sha1::hash_to_lowercase(&[]), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        // SHA-2
        assert_eq!(Sha224::hash_to_lowercase(&[]), "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f");
        assert_eq!(Sha256::hash_to_lowercase(&[]), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(Sha384::hash_to_lowercase(&[]), "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b");
        assert_eq!(Sha512::hash_to_lowercase(&[]), "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
        assert_eq!(Sha512Trunc224::hash_to_lowercase(&[]), "6ed0dd02806fa89e25de060c19d3ac86cabb87d6a0ddd05c333b84f4");
        assert_eq!(Sha512Trunc256::hash_to_lowercase(&[]), "c672b8d1ef56ed28ab87c3622c5114069bdd3ad7b8f9737498d0c01ecef0967a");
    }
}
