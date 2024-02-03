fn main() {
    println!("{:?}", Solution::running_sum(vec![1, 2, 3, 4]));
}

struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            result.push(sum);
        }
        result
    }
}