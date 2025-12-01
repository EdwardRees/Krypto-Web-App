use dioxus::prelude::*;
use ui::{Echo, Hero, Game};

#[component]
pub fn Home() -> Element {
    rsx! {
        Game {}
        // Hero {}
        // Echo {}
    }
}
