use chrono::{Duration, Local, NaiveDate};
use log::error;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::{prelude::*, virtual_dom::VNode};

fn initial_number() -> String {
    "TBD".to_string()
}

fn today() -> NaiveDate {
    Local::now().date_naive()
}
fn in_four_weeks() -> NaiveDate {
    add_four_weeks(today())
}

fn add_four_weeks(date: NaiveDate) -> NaiveDate {
    date + Duration::weeks(4)
}

fn nl2br(input: String) -> VNode {
    Html::from_html_unchecked(format!("<p>{}</p>", input.replace("\n", "<br/>")).into())
}

#[function_component]
fn App() -> Html {
    let logo_url_handler = use_state(String::default);
    let logo_url_value = logo_url_handler.to_string();

    let to_handler = use_state(String::default);
    let to_value = to_handler.to_string();

    let number_handler = use_state(initial_number);
    let number_value = number_handler.to_string();

    let issued_on_handler = use_state(today);
    let issued_on_value = *issued_on_handler;

    let due_on_handler = use_state(in_four_weeks);
    let due_on_value = *due_on_handler;

    let description_handler = use_state(String::default);
    let description_value =
        Html::from_html_unchecked(format!("<div>{}</div>", *description_handler).into());

    fn create_update_handler<T>(handler: T) -> Callback<InputEvent>
    where
        T: Fn(String) + 'static,
    {
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target().and_then(|t| {
                if let Some(input) = t.dyn_ref::<HtmlInputElement>() {
                    Some::<String>(input.value().into())
                } else if let Some(text_area) = t.dyn_ref::<HtmlTextAreaElement>() {
                    Some::<String>(text_area.value().into())
                } else {
                    None
                }
            }) {
                handler(input);
            }
        })
    }

    let update_url = create_update_handler(move |value| logo_url_handler.set(value));
    let update_to = create_update_handler(move |value| to_handler.set(value));
    let update_number = create_update_handler(move |value| number_handler.set(value));
    let update_description = create_update_handler(move |value| description_handler.set(value));
    let update_issued_on = {
        Callback::from(move |event: Event| {
            let target: Option<EventTarget> = event.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                match NaiveDate::parse_from_str(&input.value(), "%Y-%m-%d") {
                    Ok(date) => {
                        issued_on_handler.set(date);
                        due_on_handler.set(add_four_weeks(date));
                    }
                    Err(e) => error!("parse date: {}", e),
                };
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
                    oninput={update_url}
                    placeholder="http://example.com/logo-url.png"
                    value={logo_url_value.clone()}
                />

               <label for="number">{{ "Number" }}</label>
               <input
                   type="text"
                   name="number"
                   value={number_value.clone()}
                   oninput={update_number}
                />

               <label for="issued_on">{{ "Issued on" }}</label>
               <input
                   type="date"
                   name="issued_on"
                   value={issued_on_value.format("%Y-%m-%d").to_string()}
                   onchange={update_issued_on}
                />

               <label for="to">{{ "To" }}</label>
               <textarea
                   name="to"
                   rows="2"
                   value={to_value.clone()}
                   oninput={update_to}
                />

               <label for="description">{{ "Description" }}</label>
               <textarea
                   name="description"
                   rows="4"
                   oninput={update_description}
                />
            </div>

            <div class="preview paper">
                <img class="logo" alt="logo" src={{logo_url_value}} />
                <div class="header">
                    <div class="address">
                        <strong>{{ "berkes - Bèr Kessels"}}</strong>
                        {{ nl2br("Ooijse Bandijk 48\n6576 JE Ooij\nber@berk.es\nKVK:09157490\nBTW: NL001530822B70".to_string()) }}
                    </div>

                    <div class="address">
                        {{nl2br(to_value)}}
                    </div>
                </div>

                <div class="meta">
                    {{"#"}}{{number_value}}<br />
                    {{"Verzonden op: "}}{{ issued_on_value.format("%d-%m-%Y") }}<br />
                    {{"Te betalen voor: "}}{{ due_on_value.format("%d-%m-%Y") }}
                </div>

                <div class="description">{{ description_value }}</div>
                <table class="main-table">
                    <tr>
                        <th></th>
                        <th class="align-center">{{ "Prijs" }}</th>
                        <th class="align-center">{{ "Aantal" }}</th>
                        <th class="align-center">{{ "Bedrag €" }}</th>
                    </tr>
                </table>

                <div class="payment-details">
                    <p>{{"Graag voldoen naar: "}}<tt>{{"NL70 TRIO 0379 6282 60"}}</tt>{{" — berkes"}}</p>
                </div>

                <div class="foot">
                    <p class="sign">{{"Vriendelijke groet,"}}</p>
                    <hr />
                    <p class="signa">{{"Bèr Kessels"}}</p>
                </div>
            </div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
