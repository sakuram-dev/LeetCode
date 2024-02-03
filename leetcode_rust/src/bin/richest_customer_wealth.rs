// Problem: 1672. Richest Customer Wealth

use std::vec;

fn main() {
    let accounts = vec![vec![1, 2, 3], vec![3, 2, 1]];
    println!("{}", Solution::maximum_wealth(accounts));
}

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut max_wealth = 0;
        for i in 0..accounts.len() {
            let mut sum = 0;
            for j in 0..accounts[i].len() {
                sum += accounts[i][j];
            }
            if sum > max_wealth {
                max_wealth = sum;
            }
        }
        max_wealth
        
    }
}