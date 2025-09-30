use dioxus::prelude::*;

const NAV_BAR_CSS: Asset = asset!("/assets/styling/nav_bar.css");
#[component]
pub fn NavBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAV_BAR_CSS }
        div {
            id: "navbar",
            h2 {
                span { style: "text-shadow: 0 0 1em rgba(249, 226, 175, 1)", "✨ " }
                span { style: "text-shadow: 0 0 1em rgba(203, 166, 247, 1)", em { strong { "MY" } }, " BCA Bus App" }
                span { style: "text-shadow: 0 0 1em rgba(249, 226, 175, 1)", " ✨"}
            }
        }
    }
}
