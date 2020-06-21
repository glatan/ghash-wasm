#![recursion_limit = "2048"]

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
                            <td>{ "BLAKE-224" }</td>
                            <td>{ Blake224::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-256" }</td>
                            <td>{ Blake256::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-384" }</td>
                            <td>{ Blake384::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "BLAKE-512" }</td>
                            <td>{ Blake512::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "MD2" }</td>
                            <td>{ Md2::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "MD4" }</td>
                            <td>{ Md4::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "MD5" }</td>
                            <td>{ Md5::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-128" }</td>
                            <td>{ Ripemd128::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-160" }</td>
                            <td>{ Ripemd160::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-256" }</td>
                            <td>{ Ripemd256::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "RIPEMD-320" }</td>
                            <td>{ Ripemd320::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-0" }</td>
                            <td>{ Sha0::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-1" }</td>
                            <td>{ Sha1::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-224" }</td>
                            <td>{ Sha224::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-256" }</td>
                            <td>{ Sha256::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-384" }</td>
                            <td>{ Sha384::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-512" }</td>
                            <td>{ Sha512::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-512/224" }</td>
                            <td>{ Sha512Trunc224::hash_to_lowerhex(&self.value.as_bytes()) }</td>
                        </tr>
                        <tr>
                            <td>{ "SHA-512/256" }</td>
                            <td>{ Sha512Trunc256::hash_to_lowerhex(&self.value.as_bytes()) }</td>
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
