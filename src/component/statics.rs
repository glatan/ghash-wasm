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

pub fn footer() -> Html {
    html! {
        <footer>
            <div>
                <p>{ "This project is licensed under " }<a href="https://www.gnu.org/licenses/agpl-3.0.en.html">{ "GNU Affero General Public License, Version 3" }</a></p>
            </div>
        </footer>
    }
}

pub fn bow() -> Html {
    html! {
        <section style="text-align: center;">
            <h2>{ "ハッシュ関数がデータを処理する過程を表示するところが完成しませんでした。" } </h2>
            <h2>{ "🙇🙇🙇" } </h2>
        </section>
    }
}
