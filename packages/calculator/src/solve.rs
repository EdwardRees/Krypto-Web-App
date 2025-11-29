use crate::{ast::build_ast, eval::eval_tree};

pub fn solve(equation: &str) -> Result<f64, String> {
    let ast = build_ast(&equation)?;
    let val: f64 = eval_tree(Box::new(ast));
    Ok(val)
}
