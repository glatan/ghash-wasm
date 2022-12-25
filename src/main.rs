#![recursion_limit = "4096"]

use ghash::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{html, Component, Context, Html, Renderer};

mod component;

use component::statics::*;

macro_rules! hash_outputs {
    ($self:expr, $( $name:expr, $T:ty, $params:expr );+) => {
        html!{
            <tbody>
            $(
                <tr>
                    <td>{ $name }</td>
                    <td>{ $T::$params.hash_to_lowerhex($self.value.as_bytes()) }</td>
                </tr>
            )+
            </tbody>
        }
    };
}

struct App {
    value: String,
}

enum Msg {
    OnInput(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: String::new(),
        }
    }
    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnInput(input) => {
                self.value = input;
                true
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_input = ctx.link().callback(|e: InputEvent| {
            let target = e.target().expect("event target not found");
            Msg::OnInput(target.unchecked_into::<HtmlInputElement>().value())
        });

        html! {
            <>
            { header() }
            <main>
            <textarea
                oninput={on_input}
            >
            </textarea>
                <table>
                    <thead>
                        <tr>
                            <th>{ "Algorithm" }</th>
                            <th>{ "Digest" }</th>
                        </tr>
                    </thead>
                    { hash_outputs!(self,
                        "BLAKE-28", Blake28, default();
                        "BLAKE-32", Blake32, default();
                        "BLAKE-48", Blake48, default();
                        "BLAKE-64", Blake64, default();
                        "BLAKE-224", Blake224, default();
                        "BLAKE-256", Blake256, default();
                        "BLAKE-384", Blake384, default();
                        "BLAKE-512", Blake512, default();
                        "BLAKE2s", Blake2s, default();
                        "BLAKE2b", Blake2b, default();
                        "EDON-R224", EdonR224, default();
                        "EDON-R256", EdonR256, default();
                        "EDON-R384", EdonR384, default();
                        "EDON-R512", EdonR512, default();
                        "Keccak-f[200](r=40, c=160)", KeccakF200, new(40, 160, 64);
                        "Keccak-f[400](r=144, c=256)", KeccakF400, new(144, 256, 64);
                        "Keccak-f[400](r=240, c=160)", KeccakF400, new(240, 160, 64);
                        "Keccak-f[800](r=288, c=512)", KeccakF800, new(288, 512, 64);
                        "Keccak-f[800](r=544, c=256)", KeccakF800, new(544, 256, 64);
                        "Keccak-f[800](r=640, c=160)", KeccakF800, new(640, 160, 64);
                        "Keccak-224", Keccak224, default();
                        "Keccak-256", Keccak256, default();
                        "Keccak-384", Keccak384, default();
                        "Keccak-512", Keccak512, default();
                        "MD2", Md2, default();
                        "MD4", Md4, default();
                        "MD5", Md5, default();
                        "RIPEMD-128", Ripemd128, default();
                        "RIPEMD-160", Ripemd160, default();
                        "RIPEMD-256", Ripemd256, default();
                        "RIPEMD-320", Ripemd320, default();
                        "SHA-0", Sha0, default();
                        "SHA-1", Sha1, default();
                        "SHA-224", Sha224, default();
                        "SHA-256", Sha256, default();
                        "SHA-384", Sha384, default();
                        "SHA-512", Sha512, default();
                        "SHA-512/224", Sha512Trunc224, default();
                        "SHA-512/256", Sha512Trunc256, default();
                        "SHA3-224", Sha3_224, default();
                        "SHA3-256", Sha3_256, default();
                        "SHA3-384", Sha3_384, default();
                        "SHA3-512", Sha3_512, default();
                        "SHAKE128", Shake128, default();
                        "SHAKE256", Shake256, default()
                    )}
                </table>
            </main>
            { footer() }
            </>
        }
    }
}

fn main() {
    Renderer::<App>::new().render();
}
