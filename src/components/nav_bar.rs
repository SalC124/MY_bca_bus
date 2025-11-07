use dioxus::prelude::*;

const NAV_BAR_CSS: Asset = asset!("/assets/styling/nav_bar.css");
#[component]
pub fn NavBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAV_BAR_CSS }
        div {
            id: "navbar",
            h2 {
                span {
                    em { "MY" }, " BCA Bus App",
                    span {
                        style: "font-size: 0.6em",
                        " • by "
                        a {
                            href: "https://github.com/salc124",
                            "SalC124"
                        }
                    }
                }
            }
        }
    }
}
