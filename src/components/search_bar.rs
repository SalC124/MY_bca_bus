use dioxus::prelude::*;

use crate::AppStates;

const SEARCH_BAR_CSS: Asset = asset!("/assets/styling/search_bar.css");

#[component]
pub fn SearchBar() -> Element {
    let mut search_query = use_context::<AppStates>().search_query;
    let mut text: Signal<String> = use_signal(|| "".to_string());

    let oninput = move |evt: Event<FormData>| {
        text.set(evt.value());

        let val: Option<String> = match evt.value().as_str() {
            "" => None,
            text => Some(text.to_string()),
        };
        search_query.set(val);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: SEARCH_BAR_CSS }

        div {
            id: "text-input-container",
            form {
                input {
                    id: "text-input",
                    class: "inputs",
                    type: "text",
                    placeholder: "enter town name",
                    // style: "width: {text_input_width}",
                    name: "query",
                    value: text,
                    oninput
                }
            }
            p { "{search_query:?}" }
        }
    }
}
