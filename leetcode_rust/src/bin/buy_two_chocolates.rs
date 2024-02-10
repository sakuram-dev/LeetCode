use std::{result, vec};

// 2706. Buy Two Chocolates
fn main () {
    let prices = vec![1,2,2];
    let money = 3;
    println!("{}", Solution::buy_choco(prices, money));
}

struct Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut vec = prices.clone();
        
        vec.sort();

        let mut min = vec[0] + vec[1];

        if money < min {
            return money;
        } else {
            return money - min;
        }
    }
}