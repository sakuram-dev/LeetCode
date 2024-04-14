// 292. Nim Game

fn main() {
    let n = 4;
    let result = Solution::can_win_nim(n);
    println!("result = {}", result);
}

struct Solution;

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}