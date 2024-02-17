// 2423. Remove Letter To Equalize Frequency

fn main() {
    let s = "aazz";
    println!("{:?}", Solution::equal_frequency(s.to_string()));
}

struct Solution;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut counts = vec![0; 26]; // a to z
        
        // determine character counts
        word.chars().for_each(|c| counts[c as usize - 'a' as usize] += 1);
        
        // remove non-zero occurrences
        let mut sorted: Vec<i32> = counts.into_iter()
            .filter(|count| *count != 0)
            .collect();
        sorted.sort();
        
        // check whether all characters are unique
        if sorted.iter().all(|count| *count == 1) {
            return true;
        }
        
        // check removing the least frequent element
        if sorted[0] == 1 && sorted[2..].iter().all(|count| *count == sorted[1]) {
            return true;
        }
        
        // check removing the most frequent element
        let comp = sorted[sorted.len()-1] - 1;
        for i in 0..sorted.len()-1 {
            if sorted[i] != comp {
                return false;
            }
        }
        true
    }
}