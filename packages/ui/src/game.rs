use dioxus::prelude::*;
// use dioxus_logger::tracing::{info};

const GAME_CSS: Asset = asset!("/assets/styling/game.css");

#[component]
fn NumberList(nums: ReadSignal<Vec<f64>>) -> Element {
    rsx! {
        ul {
            for (idx, num) in nums.read().iter().enumerate() {
                li { key: "{idx}", "{num}" }
            }
        }
    }
}
#[component]
pub fn Game() -> Element {
    let mut response = use_signal(|| String::new());
    let mut equation = use_signal(|| String::new());
    let mut date = use_signal(|| 10.0);
    let mut nums = use_signal(|| vec![1.0, 1.0, 1.0, 1.0, 1.0]);

    let server_response = use_server_future(move || async move {
        let server_response = api::setup_nums().await.unwrap();
        date.set(server_response.clone().0);
        nums.set(server_response.clone().1);
        server_response
    });

    rsx! {
        document::Link { rel: "stylesheet", href: GAME_CSS }
        div {
            id: "game",
            h3 { "Game" }
            h4 { 
                span { "Solve: "} 
                span {
                    class: "number",
                    "{date}"
                } 
            }
            NumberList{ nums: nums }
            div {
                id: "input_box",
                input {
                    placeholder: "Type your equation here...",
                    oninput: move |event| {
                        equation.set(event.value());
                    },
                }
                button {
                    onclick: move |_event| {
                        // info!("New nums: {:?}", &server_response);
                        let eq_val = equation.peek().clone();
                        let res = &server_response.clone().ok();
                        let value = match res {
                            Some(val) => {
                                let value = val.value().clone();
                                match &*value.read_unchecked() {
                                    Some(val) => &val.clone(),
                                    None => &(0.0, vec![0.0, 0.0, 0.0, 0.0, 0.0])
                                }
                            },
                            None => &(0.0, vec![0.0, 0.0, 0.0, 0.0, 0.0])
                        };
                        let nums_val = value.1.clone();
                        let date_val = value.0;

                        // info!("Button - equation: {}, nums: {:?}, date: {:?}", eq_val, nums_val, date_val);
                        
                        spawn(async move {
                            // info!("Spawn - about to call API with nums: {:?}", nums_val);
                            match api::calculate(eq_val, nums_val.to_owned(), date_val.to_owned()).await {
                                Ok(result) => {
                                    // info!("API Result: {:?}", result);
                                    response.set(result);
                                }
                                Err(e) => {
                                    // info!("API Error: {:?}", e);
                                    response.set(format!("Error: {}", e));
                                }
                            }
                        });
                    },
                    "Send"
                }
            }
            if !response.read().is_empty() {
                p {
                    // "Response: "
                    i { "{response}" }
                }
            }            
        }
    }
}
