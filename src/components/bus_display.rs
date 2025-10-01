use dioxus::prelude::*;
use std::{collections::HashMap, time::Duration};
use tokio::time::sleep;

use crate::AppStates;

const BUS_DISPLAY_CSS: Asset = asset!("/assets/styling/bus_display.css");

#[component]
pub fn BusDisplay() -> Element {
    let app_states = use_context::<AppStates>();
    let mut bus_vec = app_states.bus_vec;
    let mut notification = app_states.notification;

    let search_query = app_states.search_query;

    let sheet_id = "1S5v7kTbSiqV8GottWVi5tzpqLdTrEgWEY4ND4zvyV3o";

    let fetch_buses = move || {
        spawn(async move {
            let url =
                format!("https://docs.google.com/spreadsheets/d/{sheet_id}/export?format=csv");

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
        });
    };

    let onsubmit = move |_: FormEvent| {
        fetch_buses();
    };

    let filtered_buses = match search_query.read().clone() {
        Some(query) => {
            let buses = bus_vec.read().clone();
            buses
                .iter()
                .filter(|(name, _)| name.to_lowercase().contains(&query.to_lowercase()))
                .cloned()
                .collect()
        }
        None => bus_vec.read().clone(),
    };
    use_effect(move || {
        spawn(async move {
            loop {
                fetch_buses();
                sleep(Duration::from_secs(30)).await;
            }
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: BUS_DISPLAY_CSS }
        div {
            id: "bus-display",
            div {
                id: "bus-list-container",
                div {
                    id: "bus-list-header",
                    span { "Bus List" }
                    form {
                        id: "fetch-buses",
                        onsubmit,
                        button { "Refresh Buses" },
                    }
                }
                table {
                    id: "bus-list",
                    tbody {
                        for (name, code) in filtered_buses.clone() {
                            tr {
                                td { class: "location-name", "{name}" } td { class: "location-code", "{code}" }
                            }
                        }
                        if !bus_vec.read().clone().is_empty() && filtered_buses.clone().is_empty() {
                            p { style: "color: var(--red)", "No towns matched your query" }
                        }
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
