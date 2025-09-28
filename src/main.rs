use std::collections::HashMap;

use dioxus::prelude::*;
mod components;

use crate::components::{BusDisplay, Notifications};

#[derive(Clone, Debug)]
pub struct AppStates {
    bus_map: Signal<Option<HashMap<String, String>>>,
    notification: Signal<Option<String>>,
}

#[component]
fn App() -> Element {
    let mut app_states: AppStates = use_context_provider(|| AppStates {
        bus_map: Signal::new(None),
        notification: Signal::new(None),
    });

    rsx! {
        Notifications {}
        h2 { em { "MY" }, " BCA Bus App" }
        BusDisplay {}
    }
}


fn main() {
    dioxus::launch(App);
}
