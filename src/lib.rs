mod digest;

use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

use digest::{Digest, HashType};

struct Form {
    link: ComponentLink<Self>,
    value: String,
}

enum Msg {
    Input(String),
}

impl Component for Form {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            value: String::new(),
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value) => self.value = value,
        }
        true
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        let supported_types = [
            HashType::Blake224,
            HashType::Blake256,
            HashType::Blake384,
            HashType::Blake512,
            HashType::Md2,
            HashType::Md4,
            HashType::Md5,
            HashType::Ripemd128,
            HashType::Ripemd160,
            HashType::Ripemd256,
            HashType::Ripemd320,
            HashType::Sha0,
            HashType::Sha1,
            HashType::Sha224,
            HashType::Sha256,
            HashType::Sha384,
            HashType::Sha512,
            HashType::Sha512Trunc224,
            HashType::Sha512Trunc256,
        ];
        let digest = |t| {
            html! {
                <Digest
                    hash_type=t
                    oninput=self.link.callback(Msg::Input)
                />
            }
        };
        html! {
            <>
                <form>
                    <textarea
                    value=&self.value
                    oninput=self.link.callback(|e: InputData| Msg::Input(e.value))>
                    </textarea>
                </form>
                <table>
                    <thead>
                        <tr>
                            <th>{ "アルゴリズム" }</th>
                            <th>{ "値" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { supported_types.iter().map(|t| digest(t)).collect::<Html>() }
                    </tbody>
                </table>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::initialize();
    yew::start_app::<Form>();
}
