// 2465. Number of Distinct Averages

use std::collections::HashSet;

fn main() {
    let a = vec![3, 1, 4, 2];
    println!("{}", Solution::distinct_averages(a));
}

struct Solution;

impl Solution {
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut res: HashSet<i32> = HashSet::new();

        let mut nums: Vec<i32> = nums;
        nums.sort();

        let mut l: usize = 0;
        let mut r: usize = nums.len() -1;

        while l < r {
            res.insert(nums[l] + nums[r]);
            l+=1;
            r-=1;
        }

        res.len() as i32
    }
}