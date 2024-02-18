// 1816. Truncate Sentence

use std::vec;

fn main() {
    let s = "Hello how are you Contestant";
    let k = 4;
    println!("{:?}", Solution::truncate_sentence(s.to_string(), k));
}

struct Solution;

impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let split_s: Vec<&str> = s.split_whitespace().collect();
        let mut result = vec![];
        for i in 0..k as usize{
            result.push(split_s[i]);
        }
        result.join(" ")
    }
}