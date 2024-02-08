// 344. Reverse String

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    println!("{:?}", Solution::reverse_string(&mut s));
}

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut last = s.len() - 1;
        for i in 0..s.len() / 2 {
            s.swap(i, last);
            last -= 1;
        }
    }
}