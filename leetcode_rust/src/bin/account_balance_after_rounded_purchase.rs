// 2806. Account Balance After Rounded Purchase
fn main() {
    let purchase_amount = 11;
    println!("{}", Solution::account_balance_after_purchase(purchase_amount));
}

struct Solution;

impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        100 - (purchase_amount + 5) / 10 * 10
    }
}