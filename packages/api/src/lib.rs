//! This crate contains all shared fullstack server functions.
use dioxus::prelude::*;
use calculator::{krypto::{valid_input, setup}, ast::build_ast, solve::solve, eval::eval_tree_nums};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct AuthCredentials {
    user_id: String, // UUID
    username: String,
    token: String // jwt
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
pub async fn signup(username: String, password: String) -> Result<String, ServerFnError> {
    Ok(String::from("Signed up"))
}

