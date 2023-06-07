use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let to_handler = use_state(String::default);
    let to_value = to_handler.to_string();

    let description_handler = use_state(String::default);
    let description_value =
        Html::from_html_unchecked(format!("<div>{}</div>", *description_handler).into());

    let logo_url_handler = use_state(String::default);
    let logo_url_value = (*logo_url_handler).clone();

    let update_url = {
        Callback::from(move |event: Event| {
            let input = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                logo_url_handler.set(input.value());
            }
        })
    };

    let update_description = {
        Callback::from(move |event: Event| {
            let input = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            if let Some(input) = input {
                description_handler.set(input.value());
            }
        })
    };

    let update_to = {
        Callback::from(move |event: Event| {
            let input = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            if let Some(input) = input {
                to_handler.set(input.value());
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
                    type="url"
                    onchange={update_url}
                    placeholder="http://example.com/logo-url.png"
                    value={logo_url_value.clone()}
                />

               <label for="to">{{ "To" }}</label>
               <textarea
                   name="to"
                   rows="2"
                   value={to_value.clone()}
                   onchange={update_to}
                />

               <label for="description">{{ "Description" }}</label>
               <textarea
                   name="description"
                   rows="4"
                   onchange={update_description}
                />
            </div>
            <div class="preview paper">
                <img class="logo" src={ logo_url_value } alt="logo" />
                <br/>
                <strong>{{"to:"}}</strong><br/>
                <pre>{{ to_value }}</pre>

                <p>{{ description_value }}</p>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
