use dioxus::prelude::*;
use std::collections::HashMap;

use crate::AppStates;

const BUS_DISPLAY_CSS: Asset = asset!("/assets/styling/bus_display.css");

#[component]
pub fn BusDisplay() -> Element {
    let app_states = use_context::<AppStates>();
    let mut bus_map = app_states.bus_map;
    let mut notification = app_states.notification;

    let search_query = app_states.search_query;

    let sheet_id = "1S5v7kTbSiqV8GottWVi5tzpqLdTrEgWEY4ND4zvyV3o";

    let onsubmit = move |_: FormEvent| async move {
        let url = format!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=csv");

        let res = ureq::get(url).call();
        match res {
            Ok(mut success) => {
                let res_body = success.body_mut().read_to_string();
                match res_body {
                    Ok(text) => {
                        bus_map.set(Some(parse_town_locations(&text)));
                    }
                    Err(err) => notification.set(Some(format!("Error: {err}"))),
                }
            }
            Err(err) => notification.set(Some(format!("Error: {err}"))),
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: BUS_DISPLAY_CSS }
        div {
            id: "bus-display",
            match bus_map.as_ref() {
                Some(bus_map) => {
                    let mut sorted: Vec<_> = bus_map.iter().collect();
                    sorted.sort_by_key(|(name, _)| name.to_lowercase());

                    rsx! {
                        div {
                            id: "bus-list-container",
                            ul {
                                id: "bus-list",
                                for (name, code) in sorted {
                                    li {
                                        span { class: "location-name", "{name}: " }
                                        span { class: "location-code", "{code}" }
                                    }
                                }
                            }
                        }
                    }
                }
                None => rsx! {
                    div {
                        id: "fetch-buses",
                        form {
                            onsubmit,
                            button { "get bus locs" },
                        }
                    }
                },
            }
        }
    }
}
fn parse_town_locations(csv: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in csv.lines() {
        // since comma-separated, split by comma and trim
        let cells: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
        let mut i = 0;
        // go through each line and check each cell (exclude the first line)
        while i + 1 < cells.len() {
            let town = cells[i];
            let loc = cells[i + 1];
            if !town.is_empty()
                && !loc.is_empty()
                // check that char[0] is Some and is a letter
                && loc.chars().next().unwrap_or(' ').is_ascii_alphabetic()
            {
                map.insert(town.to_string(), loc.to_string());
            }
            i += 2;
        }
    }

    map
}
