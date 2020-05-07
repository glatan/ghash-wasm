use ghash::*;
use yew::{html, Callback, Component, ComponentLink, Html, InputData, Properties, ShouldRender};

#[derive(Clone, Copy, PartialEq)]
pub enum HashType {
    Blake224,
    Blake256,
    Blake384,
    Blake512,
    Md2,
    Md4,
    Md5,
    Ripemd128,
    Ripemd160,
    Ripemd256,
    Ripemd320,
    Sha0,
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha512Trunc224,
    Sha512Trunc256,
}

pub struct Digest {
    link: ComponentLink<Self>,
    value: String,
    hash_type: HashType,
    oninput: Callback<String>,
}

pub enum Msg {
    ParentInput(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub initial: String,
    #[prop_or(HashType::Md2)]
    pub hash_type: HashType,
    pub oninput: Callback<String>,
}

impl Component for Digest {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            hash_type: props.hash_type,
            value: hash_to_lowerhex(&props.hash_type, ""),
            oninput: props.oninput,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ParentInput(message) => self.value = self.update_digest(&message),
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
    fn view(&self) -> Html {
        html! {
            <tr>
                <td>{ self.get_hash_function_name() }</td>
                <td>{ &self.value }</td>
            </tr>
        }
    }
}

fn hash_to_lowerhex(hash_type: &HashType, message: &str) -> String {
    let bytes = message.as_bytes();
    match hash_type {
        HashType::Blake224 => Blake224::hash_to_lowerhex(bytes),
        HashType::Blake256 => Blake256::hash_to_lowerhex(bytes),
        HashType::Blake384 => Blake384::hash_to_lowerhex(bytes),
        HashType::Blake512 => Blake512::hash_to_lowerhex(bytes),
        HashType::Md2 => Md2::hash_to_lowerhex(bytes),
        HashType::Md4 => Md4::hash_to_lowerhex(bytes),
        HashType::Md5 => Md5::hash_to_lowerhex(bytes),
        HashType::Ripemd128 => Ripemd128::hash_to_lowerhex(bytes),
        HashType::Ripemd160 => Ripemd160::hash_to_lowerhex(bytes),
        HashType::Ripemd256 => Ripemd256::hash_to_lowerhex(bytes),
        HashType::Ripemd320 => Ripemd320::hash_to_lowerhex(bytes),
        HashType::Sha0 => Sha0::hash_to_lowerhex(bytes),
        HashType::Sha1 => Sha1::hash_to_lowerhex(bytes),
        HashType::Sha224 => Sha224::hash_to_lowerhex(bytes),
        HashType::Sha256 => Sha256::hash_to_lowerhex(bytes),
        HashType::Sha384 => Sha384::hash_to_lowerhex(bytes),
        HashType::Sha512 => Sha512::hash_to_lowerhex(bytes),
        HashType::Sha512Trunc224 => Sha512Trunc224::hash_to_lowerhex(bytes),
        HashType::Sha512Trunc256 => Sha512Trunc256::hash_to_lowerhex(bytes),
    }
}

impl Digest {
    fn get_hash_function_name(&self) -> &str {
        match self.hash_type {
            HashType::Blake224 => "BLAKE-224",
            HashType::Blake256 => "BLAKE-256",
            HashType::Blake384 => "BLAKE-384",
            HashType::Blake512 => "BLAKE-512",
            HashType::Md2 => "MD2",
            HashType::Md4 => "MD4",
            HashType::Md5 => "MD5",
            HashType::Ripemd128 => "RIPEMD-128",
            HashType::Ripemd160 => "RIPEMD-160",
            HashType::Ripemd256 => "RIPEMD-256",
            HashType::Ripemd320 => "RIPEMD-320",
            HashType::Sha0 => "SHA-0",
            HashType::Sha1 => "SHA-1",
            HashType::Sha224 => "SHA-224",
            HashType::Sha256 => "SHA-256",
            HashType::Sha384 => "SHA-384",
            HashType::Sha512 => "SHA-512",
            HashType::Sha512Trunc224 => "SHA-512/224",
            HashType::Sha512Trunc256 => "SHA-512/256",
        }
    }
    fn update_digest(&self, message: &str) -> String {
        hash_to_lowerhex(&self.hash_type, message)
    }
}
