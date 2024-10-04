// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 使用 match 来处理 total_cost 的结果
    let cost = match total_cost(pretend_user_input) {
        Ok(cost) => cost, // 如果解析成功，将值绑定到 cost 变量
        Err(_) => {
            println!("Failed to parse the user input.");
            return; // 如果解析失败，打印错误信息并返回
        }
    };

    // 判断是否有足够的 tokens 来支付 cost
    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
