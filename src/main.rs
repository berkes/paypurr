use log::debug;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let logo_url_handler = use_state(String::default);
    let logo_url_value = (*logo_url_handler).clone();
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            debug!("value: {}", value);
            counter.set(value);
        }
    };
    let update_url = {
        let url = logo_url_handler.clone();

        Callback::from(move |event: Event| {
            let input = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                debug!("{:#?}", input);
                url.set(input.value().to_string());
            }
        })
    };

    html! {
        <div class="wrapper">
            <h1 id="brand">
                <img id="logo" src="logo.svg" />
                <span class="name">{{ "PayPurr" }}</span>
            </h1>
            <div class="input">
                <input
                    type="text"
                    onchange={update_url}
                    placeholder="http://example.com/logo-url.png"
                    value={logo_url_value.clone()}
                /> <br />
                <button {onclick}>{ "+1" }</button>
            </div>
            <div class="preview paper">
                <img class="logo" src={ logo_url_value } alt="logo" />
                <p>{ *counter }</p>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
