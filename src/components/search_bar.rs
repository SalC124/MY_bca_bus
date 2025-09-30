use dioxus::prelude::*;

use crate::AppStates;

const SEARCH_BAR_CSS: Asset = asset!("/assets/styling/search_bar.css");

#[component]
pub fn SearchBar() -> Element {
    let mut text: Signal<String> = use_signal(|| "".to_string());
    let onchange = move |evt: Event<FormData>| {
        text.set(evt.value());
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
                    onchange
                }
            }
        }
    }
}
