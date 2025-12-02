use crate::Route;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/login.css");

#[component]
pub fn Login() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS}

        div {
            id: "login",

            h1 { "Login" }

            // // Content
            // h1 { "This is blog #{id}!" }
            // p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
            //
            // Navigation links
            // Link {
            //     to: Route::Blog { id: id - 1 },
            //     "Previous"
            // }
            // span { " <---> " }
            // Link {
            //     to: Route::Blog { id: id + 1 },
            //     "Next"
            // }
        }
    }
}


