//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use calculator::{krypto::{valid_input, setup}, ast::build_ast, solve::solve, eval::eval_tree_nums};
use serde::{Serialize, Deserialize};
use auth::{signup as signup_handler};
use cookie::{Cookie, SameSite};
use dioxus::prelude::dioxus_fullstack::{
    response::{Response},
    http::{StatusCode, Uri, 
        header::{self, HeaderMap, HeaderName, SET_COOKIE}
    }
};


#[derive(Serialize, Deserialize)]
struct Auth {
    user_id: String,
    access_token: String
}

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[post("/api/setup")]
pub async fn setup_nums() -> Result<(f64, Vec<f64>), ServerFnError> {
    Ok(setup())
}


#[post("/api/calculate")]
pub async fn calculate(input: String, nums: Vec<f64>, date: f64) -> Result<String, ServerFnError> {
    let ast = match build_ast(&input){
        Ok(node) => node,
        Err(error) => {
            return Ok(format!("Error: {error}"));
        }
    };
    let mut answered_nums: Vec<f64> = Vec::new();
    eval_tree_nums(Box::new(ast), &mut answered_nums);
    if !valid_input(&nums, &answered_nums){
        return Ok(format!("Invalid numbers used. {:?}\nValid numbers: {:?}", answered_nums, nums));
    }

    let solution = match solve(&input) {
        Ok(num) => num,
        Err(err) => 0.0
    };

    if solution == date {
        Ok(String::from("Correct!"))
    } else {
        Ok(format!("Good try! You answered: {}.", solution))
    }
}

#[post("/api/signup")]
pub async fn signup(email: String, password: String) -> Result<Response, ServerFnError> {
    let auth = match signup_handler(email, password) {
        Ok(creds) => {
            let refresh_cookie = Cookie::new("refresh_token", creds.refresh_token.to_string())
                .path("/")
                .http_only(true)
                .secure(true)
                .max_age(Duration::days(30))
                .same_site(SameSite)
                .finish();
            let mut header_map = HeaderMap::new();
            header_map.insert([(SET_COOKIE, refresh_cookie.to_string().unwrap())]);
            return Ok((header_map, Auth { user_id: creds.user_id, access_token: creds.access_token }));
        },
        Err(error) => {
            return Err(error.into());
        }
    };
}

