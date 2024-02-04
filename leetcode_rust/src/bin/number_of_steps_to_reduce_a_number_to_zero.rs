// 1342. Number of Steps to Reduce a Number to Zero

fn main() {
    let num = 14;
    println!("{}", Solution::number_of_steps(num));
}

struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut val = num;
        let mut count = 0;
        while 0 < val {
            if val % 2 == 0 {
                val = val / 2;
            } else {
                val = val - 1;
            }

            count += 1;
        }
        count
    }
}