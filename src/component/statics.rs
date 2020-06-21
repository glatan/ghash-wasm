use yew::{html, Html};

pub fn header() -> Html {
    html! {
        <header>
            <div class="title">
                <p>{ "Ghash Wasm" }</p>
            </div>
            <div class="links">
                <p><a href="https://gitlab.com/glatan/ghash-wasm">{ "Source Code" }</a></p>
            </div>
        </header>
    }
}
