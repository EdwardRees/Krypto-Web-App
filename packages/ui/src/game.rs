use dioxus::prelude::*;
use dioxus_logger::tracing::{info};

const GAME_CSS: Asset = asset!("/assets/styling/echo.css");

#[component]
fn NumberList(nums: ReadSignal<Vec<f64>>, updated: WriteSignal<bool>, new_nums: WriteSignal<Vec<f64>>) -> Element {
    let s_nums = nums.read().clone();
    if s_nums[0] != 1.0 && s_nums[1] != 1.0 && s_nums[2] != 1.0 && s_nums[3] != 1.0 && s_nums[4] != 1.0 {
        updated.set(true);
        s_nums.iter().for_each(|n| new_nums.push(*n));
    } else {
        updated.set(false);
    }
    rsx! {
        ul {
            for (idx, num) in nums.read().iter().enumerate() {
                li { key: "{idx}", "{num}" }
            }
        }
    }
}
// #[component]
// pub fn Game() -> Element {
//     let mut response = use_signal(|| String::new());
//     let mut equation = use_signal(|| String::new());
//     let mut date = use_signal(|| 10.0);
//     let mut nums = use_signal(|| vec![1.0, 1.0, 1.0, 1.0, 1.0]);
//     let mut updated: Signal<bool> = use_signal(|| false);
//     let mut active_nums: Signal<Vec<f64>> = use_signal(|| Vec::new());
//
//     let _ = use_server_future(move || async move {
//         let server_response = api::setup_nums().await.unwrap();
//         date.set(server_response.0);
//         nums.set(server_response.1);
//         info!("numbers set: {:?}", nums);
//     });
//     rsx! {
//         document::Link { rel: "stylesheet", href: GAME_CSS }
//         div {
//             id: "game",
//             h3 { "Game" }
//             h4 { "Solve: {date}" }
//             NumberList{ nums: nums, updated: updated }
//             input {
//                 placeholder: "Type your equation here...",
//                 oninput:  move |event| {
//                     equation.set(event.value());
//                     active_nums.set(nums.read().clone());
//                     info!("Active_nums: {:?}", active_nums);
//                 },
//             }
//             button {
//                 onclick: move |_event| {
//                     // let eq = equation.read().clone();
//                     // let current_nums = nums.read().clone();
//                     // let current_date = date.read().clone();
//                     let eq = equation();
//                     let current_nums = nums();
//                     let current_date = date();
//                     info!("Current nums: {:?}", current_nums);
//                     spawn(async move {
//                         info!("Current nums in spawn: {:?}", current_nums);
//                         match api::calculate(eq, current_nums, current_date).await {
//                             Ok(result) => {
//                                 info!("{:?}", result);
//                                 response.set(result);
//                             }
//                             Err(e) => response.set(format!("Error: {}", e)),
//                         }
//                     });
//                 },
//                 "Send"
//             }
//             if !response.read().is_empty() {
//                 p {
//                     "Server echoed: "
//                     i { "{response}" }
//                     p { "{active_nums()[0]}"}
//                 }
//             }            
//         }
//     }
// }
#[component]
pub fn Game() -> Element {
    let mut response = use_signal(|| String::new());
    let mut equation = use_signal(|| String::new());
    let mut date = use_signal(|| 10.0);
    let mut nums = use_signal(|| vec![1.0, 1.0, 1.0, 1.0, 1.0]);
    let mut updated: Signal<bool> = use_signal(|| false);
    let mut new_nums: Signal<Vec<f64>> = use_signal(|| Vec::new());

    let server_response = use_server_future(move || async move {
        let server_response = api::setup_nums().await.unwrap();
        date.set(server_response.clone().0);
        nums.set(server_response.clone().1);
        // info!("Server future complete - numbers set: {:?}", server_response.1);
        server_response
    });

    rsx! {
        document::Link { rel: "stylesheet", href: GAME_CSS }
        div {
            id: "game",
            h3 { "Game" }
            h4 { "Solve: {date}" }
            NumberList{ nums: nums, updated: updated, new_nums: new_nums }
            input {
                placeholder: "Type your equation here...",
                oninput: move |event| {
                    equation.set(event.value());
                },
            }
            button {
                onclick: move |_event| {
                    // Use a resource to ensure fresh reads
                    info!("New nums: {:?}", &server_response);
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

                    // let nums_val = nums.peek().clone();
                    // let nums_val = server_response.unwrap().peek().0.peek().clone();
                    // let date_val = *date.peek();
                    // let date_val = server_response.1.peek();
                    
                    info!("Button - equation: {}, nums: {:?}, date: {:?}", eq_val, nums_val, date_val);
                    
                    spawn(async move {
                        info!("Spawn - about to call API with nums: {:?}", nums_val);
                        match api::calculate(eq_val, nums_val.to_owned(), date_val.to_owned()).await {
                            Ok(result) => {
                                info!("API Result: {:?}", result);
                                response.set(result);
                            }
                            Err(e) => {
                                info!("API Error: {:?}", e);
                                response.set(format!("Error: {}", e));
                            }
                        }
                    });
                },
                "Send"
            }
            if !response.read().is_empty() {
                p {
                    "Response: "
                    i { "{response}" }
                }
            }            
        }
    }
}
