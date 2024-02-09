// 1323. Maximum 69 Number

fn main() {
    let num = 9669;
    println!("{}", Solution::maximum69_number(num));
}

struct Solution;

impl Solution {
    pub fn maximum69_number (num: i32) -> i32 {
        let mut n = num;
        let mut num_vec = Vec::new();

        while n != 0 {
            num_vec.push(n % 10);
            n /= 10;
        }
        num_vec.reverse();

        for i in 0..num_vec.len() {
            if num_vec[i] == 6 {
                num_vec[i] = 9;
                break;
            }
        }

        let s: String = num_vec.iter().map(|i| i.to_string()).collect();
        s.parse::<i32>().unwrap()
    }
}