use dioxus::prelude::*;
use reqwest;

const GAME_CSS: Asset = asset!("/assets/styling/game.css");

#[component]
pub fn Game() -> Element {
    let mut input_response = use_signal(|| String::new());
    let mut equation = use_signal(|| String::new());
    let mut date = use_signal(|| 10.0);
    let mut nums = use_signal(|| Vec::new());

    let _ = use_server_future(move || async move {
        let server_response = api::setup_nums().await.unwrap();
        date.set(server_response.0);
        nums.set(server_response.1);
    });

    rsx! {
        document::Link { rel: "stylesheet", href: GAME_CSS }
        div {
            id: "game",
            h3 { 
                "Solve: {date}" 
            }
            ul {
                for num in nums() {
                    li { "{num}" }
                }
            }
            // div {
                label { "Expression:" }
                input {
                    placeholder: "Enter your expression here: ",
                    oninput: move |evt| {
                        equation.set(evt.value());
                    }
                }
                // button {
                //     onclick: move |evt| {
                //         evt.prevent_default();
                //         input_response.set(equation());
                //     },
                //     "Submit"
                // }
            // }
            if !equation().is_empty() {
                p {
                    "Value: {equation()}"
                }
            }
        }
    }
}
