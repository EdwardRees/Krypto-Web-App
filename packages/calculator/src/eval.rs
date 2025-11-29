use crate::ast::ASTNode;

pub fn eval_tree_nums(node: Box<ASTNode>, nums: &mut Vec<f64>) {
    match *node {
        ASTNode::Number(v) => {
            nums.push(v);
        },
        ASTNode::BinaryOp { left, right, .. } => {
            eval_tree_nums(left, nums);
            eval_tree_nums(right, nums);
        },
        _ => {}
    }
}

fn eval_tree_expr(node: Box<ASTNode>) -> f64 {
    let np1: Box<ASTNode>;
    let np2: Box<ASTNode>;

    let mut v1: f64 = 0.0;
    let mut result: f64 = 0.0;

    match *node {
        ASTNode::Number(v) => {
            v1 = v;
        },
        ASTNode::UnaryOp { operator, operand } => {
            np1 = operand;
            v1 = eval_tree_expr(np1);
            if operator == String::from("-") {
                v1 = -(v1);
            }
        },
        ASTNode::BinaryOp { operator, left, right } => {
            np1 = left;
            np2 = right;
            v1 = eval_tree_expr(np1);
            let v2 = eval_tree_expr(np2);
            if operator == String::from("+") {
                result = v1 + v2;
            } else if operator == String::from("-") {
                result = v1 - v2;
            } else if operator == String::from("*") {
                result = v1 * v2;
            } else if operator == String::from("/") {
                result = v1 / v2;
            } else if operator == String::from("^") {
                result = v1.powf(v2);
            }
            v1 = result;
        },
        _ => {}
    }

    v1
}

pub fn eval_tree(node: Box<ASTNode>) -> f64 {
    eval_tree_expr(node)
}
