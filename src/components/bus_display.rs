use dioxus::prelude::*;
use std::collections::HashMap;

use crate::AppStates;

#[component]
pub fn BusDisplay() -> Element {
    let mut app_states = use_context::<AppStates>();
    let bus_map = app_states.bus_map.read().clone();
    let response = app_states.response.read().clone();

    let sheet_id = "1S5v7kTbSiqV8GottWVi5tzpqLdTrEgWEY4ND4zvyV3o";

    let onsubmit = move |_: FormEvent| async move {
        let url = format!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=csv");

        let res = ureq::get(url).call();
        match res {
            Ok(mut success) => {
                let res_body = success.body_mut().read_to_string();
                match res_body {
                    Ok(text) => {
                        app_states.bus_map.set(Some(parse_town_locations(&text)));
                    }
                    Err(err) => app_states.response.set(format!("Error: {err}")),
                }
            }
            Err(err) => app_states.response.set(format!("Error: {err}")),
        }
    };

    return rsx! {
        match bus_map.as_ref() {
            Some(bus_map) => {
                let mut sorted: Vec<_> = bus_map.iter().collect();
                sorted.sort_by_key(|(name, _)| name.to_lowercase());

                rsx! {
                    ul {
                        for (name, code) in sorted {
                            li { "{name}: {code}" }
                        }
                    }
                }
            }
            None => rsx! {
                div {
                    id: "fetch buses",
                    form {
                        onsubmit,
                        button { "get bus locs" },
                    }
                }
                p {
                    "{response}"
                }
            },
        }
    };
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
