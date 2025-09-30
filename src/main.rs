use std::collections::HashMap;

use dioxus::prelude::*;
mod components;

use crate::components::{BusDisplay, Notifications, SearchBar, NavBar};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[derive(Clone, Debug)]
pub struct AppStates {
    bus_map: Signal<Option<HashMap<String, String>>>,
    notification: Signal<Option<String>>,
    search_query: Signal<Option<String>>,
}

#[component]
fn App() -> Element {
    let mut app_states: AppStates = use_context_provider(|| AppStates {
        bus_map: Signal::new(None),
        notification: Signal::new(None),
        search_query: Signal::new(None),
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        NavBar {}
        BusDisplay {}
        SearchBar {}
        Notifications {}
    }
}


fn main() {
    dioxus::launch(App);
}
