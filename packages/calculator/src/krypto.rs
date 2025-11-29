use crate::{solve::solve, ast::build_ast, eval::eval_tree_nums};
use std::io::{stdin, Write, stdout};
use std::env;
use rand::prelude::*;
use chrono::prelude::*;
use std::collections::BTreeMap;

fn gen_nums(amount: u32, min: i32, max: i32) -> Vec<f64> {
    let mut rng = rand::rng();
    let mut nums: Vec<f64> = Vec::new();
    for _ in 0..amount {
        nums.push(rng.random_range(min..max).into());
    }
    nums
}

fn get_date() -> f64 {
    let local: DateTime<Local> = Local::now();
    let date = local.date_naive().day() as f64;
    date 
}

fn get_vec_count(nums: &Vec<f64>) -> BTreeMap<i64, usize> {
    let mut count: BTreeMap<i64, usize> = BTreeMap::new();
    for num in nums {
        let v = *num as i64;
        match count.get_mut(&v) {
            Some(occur) => { 
                *occur += 1;
            },
            None => { 
                count.insert(v, 1);
            }
        }
    }
    count
}

fn valid_input(provided: &Vec<f64>, given: &Vec<f64>) -> bool {
    let provided_count = get_vec_count(provided);
    let given_count = get_vec_count(given);
    provided_count == given_count
}

pub fn setup() -> (f64, Vec<f64>) {
    let date: f64 = get_date();
    let nums: Vec<f64> = gen_nums(5, 1, 10);
    (date, nums)
}

pub fn command_line_game() -> Result<(), String> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(String::from("Invalid usage"));
    }

    if &args[1] == "play" {
        let date: f64 = get_date();
        let nums: Vec<f64> = gen_nums(5, 1, 10);

        for tries in 0..10 {
            let mut user = String::new();

            print!("Your numbers: ");
            for num in &nums {
                print!("{} ", num.clone() as u64);
            }
            println!();
            println!("Calculate: {}", date as u64);
            println!();
            print!(">> ");
            stdout().flush().unwrap();

            let _ = stdin().read_line(&mut user);
            let ast = build_ast(&user)?;
            let mut answered_nums: Vec<f64> = Vec::new();
            eval_tree_nums(Box::new(ast), &mut answered_nums);

            if !valid_input(&nums, &answered_nums) {
                return Err(format!("Invalid numbers used! {:?}", answered_nums));
            }

            let equation = &user;
            let solution = solve(equation)?;

            if solution == date {
                println!("Great job!");
                break;
            } else {
                println!("Good try! You answered: {}. You have {} tries left!", solution, (10 - tries - 1));
            }
        }

    } else {
        let equation = &args[1];
        let val: f64 = solve(&equation)?;
        println!("{:?}", val);
    }

    Ok(())
}
