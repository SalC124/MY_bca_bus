use std::collections::HashMap;

use dioxus::prelude::*;
mod components;

use crate::components::BusDisplay;

#[derive(Clone, Debug)]
pub struct AppStates {
    response: Signal<String>,
    bus_map: Signal<Option<HashMap<String, String>>>,
}

#[component]
fn App() -> Element {
    let mut app_states: AppStates = use_context_provider(|| AppStates {
        response: Signal::new("Loading...".to_string()),
        bus_map: Signal::new(None),
    });

    rsx! {
        h2 { em { "MY" }, " BCA Bus App" }

        BusDisplay {}
    }
}


fn main() {
    dioxus::launch(App);
}
