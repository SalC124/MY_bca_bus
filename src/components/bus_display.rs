use dioxus::{logger::tracing, prelude::*};
use std::collections::HashMap;

use crate::AppStates;

const BUS_DISPLAY_CSS: Asset = asset!("/assets/styling/bus_display.css");

#[component]
pub fn BusDisplay() -> Element {
    let app_states = use_context::<AppStates>();
    let mut bus_vec = app_states.bus_vec;
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
                        bus_vec.set(parse_town_locations(&text));
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
            if !bus_vec.is_empty() {
                div {
                    id: "bus-list-container",
                    table {
                        id: "bus-list",
                        thead { style: "font-size: 1.5em", "Bus List" }
                        tbody {
                            for (name, code) in bus_vec.read().clone() {
                                tr {
                                    td { class: "location-name", "{name}" } td { class: "location-code", "{code}" }
                                }
                            }
                        }
                    }
                }
            } else {
                div {
                    id: "fetch-buses",
                    form {
                        onsubmit,
                        button { "get bus locs" },
                    }
                }
            }
        }
    }
}

fn parse_town_locations(csv: &str) -> Vec<(String, String)> {
    let mut map: HashMap<String, String> = HashMap::new();

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

    parse_map_to_sorted(map)
}

fn parse_map_to_sorted(map: HashMap<String, String>) -> Vec<(String, String)> {
    let mut sorted: Vec<(String, String)> = map
        .into_iter()
        .map(|(name, code)| {
            (
                {
                    name.split('/')
                        .map(|s| s.trim())
                        .collect::<Vec<&str>>()
                        .join(" / ")
                },
                code,
            )
        })
        .collect();
    sorted.sort_by_key(|(name, _)| name.to_lowercase());

    sorted
}
