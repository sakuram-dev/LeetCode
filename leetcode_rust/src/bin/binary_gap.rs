// 868. Binary Gap
fn main() {
    let n = 22;
    println!("{:?}", Solution::binary_gap(n));
}

struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        let mut max_gap = 0;
        let mut gap = 0;
        let mut found_one = false;
        while n > 0 {
            if n & 1 == 1 {
                if found_one {
                    max_gap = max_gap.max(gap);
                }
                found_one = true;
                gap = 1;
            } else if found_one {
                gap += 1;
            }
            n >>= 1;
        }
        max_gap
    }
}