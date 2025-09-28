use dioxus::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::AppStates;

const NOTIFICATIONS_CSS: Asset = asset!("/assets/styling/notifications.css");

#[component]
pub fn Notifications() -> Element {
    let notification = use_context::<AppStates>().notification;
    let mut invisible = use_signal(|| true);

    use_effect(move || {
        if notification.read().is_some() {
            invisible.set(false);

            spawn({
                let mut notification = notification;
                let mut invisible = invisible;
                async move {
                    sleep(Duration::from_secs(3)).await;
                    invisible.set(true);
                    sleep(Duration::from_millis(500)).await;
                    notification.set(None);
                }
            });
        }
    });

    let notification = notification.read().clone();
    let notif_text = notification.unwrap_or("".to_string());

    rsx! {
        document::Link { rel: "stylesheet", href: NOTIFICATIONS_CSS }
        if !notif_text.is_empty() {
            div {
                class: if !*invisible.read() {"toast"} else {"toast hidden"},
                "{notif_text}"
            }
        }
    }
}
