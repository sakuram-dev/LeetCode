// 1399. Count Largest Group

fn main() {
    let n = 2;
    let result = Solution::count_largest_group(n);
    println!("result = {}", result);
}

struct  Solution;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        let mut max = 0;
        for i in 1..=n {
            let mut sum = 0;
            let mut i = i;
            while i > 0 {
                sum += i % 10;
                i /= 10;
            }
            let count = map.entry(sum).or_insert(0);
            *count += 1;
            max = max.max(*count);
        }
        map.values().filter(|&x| *x == max).count() as i32
    }
}