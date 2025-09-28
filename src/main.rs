use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let mut response = use_signal(|| "Loading...".to_string());

    let onsubmit = move |_: FormEvent| async move {
        let res = ureq::get("https://api.ipify.org/?format=json")
            .call()
            .unwrap()
            .body_mut()
            .read_to_string();

        match res {
            Ok(text) => response.set(text),
            Err(err) => response.set(format!("Error: {err}")),
        }
    };
    rsx! {
        div {
            id: "test_ureq",
            form {
                onsubmit,
                button { "{response}" },
            }
        }

    }
}

fn main() {
    dioxus::launch(App);
}
