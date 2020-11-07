#![recursion_limit = "4096"]

mod component;

use component::statics::*;

use ghash::*;
use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

struct App {
    link: ComponentLink<Self>,
    value: String,
}

enum Msg {
    OnInput(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::new(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OnInput(input) => {
                self.value = input;
                true
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <>
            { header() }
            <main>
            <textarea
                value=&self.value
                oninput=self.link.callback(|m: InputData| Msg::OnInput(m.value)),
            >
            </textarea>
                <table>
                    <thead>
                        <tr>
                            <th>{ "Algorithm" }</th>
                            <th>{ "Digest" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>{ "BLAKE-28" }</td>
                            <td>{ Blake28::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-32" }</td>
                            <td>{ Blake32::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-48" }</td>
                            <td>{ Blake48::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-64" }</td>
                            <td>{ Blake64::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-224" }</td>
                            <td>{ Blake224::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-256" }</td>
                            <td>{ Blake256::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-384" }</td>
                            <td>{ Blake384::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-512" }</td>
                            <td>{ Blake512::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE2s" }</td>
                            <td>{ Blake2s::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE2b" }</td>
                            <td>{ Blake2b::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "Keccak-224" }</td>
                            <td>{ Keccak224::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "Keccak-256" }</td>
                            <td>{ Keccak256::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "Keccak-384" }</td>
                            <td>{ Keccak384::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "Keccak-512" }</td>
                            <td>{ Keccak512::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "MD2" }</td>
                            <td>{ Md2::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "MD4" }</td>
                            <td>{ Md4::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "MD5" }</td>
                            <td>{ Md5::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-128" }</td>
                            <td>{ Ripemd128::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-160" }</td>
                            <td>{ Ripemd160::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-256" }</td>
                            <td>{ Ripemd256::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-320" }</td>
                            <td>{ Ripemd320::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-0" }</td>
                            <td>{ Sha0::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-1" }</td>
                            <td>{ Sha1::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-224" }</td>
                            <td>{ Sha224::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-256" }</td>
                            <td>{ Sha256::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-384" }</td>
                            <td>{ Sha384::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-512" }</td>
                            <td>{ Sha512::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-512/224" }</td>
                            <td>{ Sha512Trunc224::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-512/256" }</td>
                            <td>{ Sha512Trunc256::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA3-224" }</td>
                            <td>{ Sha3_224::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA3-256" }</td>
                            <td>{ Sha3_256::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA3-384" }</td>
                            <td>{ Sha3_384::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA3-512" }</td>
                            <td>{ Sha3_512::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHAKE128" }</td>
                            <td>{ Shake128::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHAKE256" }</td>
                            <td>{ Shake256::default().hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                    </tbody>
                </table>
            </main>
            { footer() }
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::initialize();
    yew::start_app::<App>();
}
