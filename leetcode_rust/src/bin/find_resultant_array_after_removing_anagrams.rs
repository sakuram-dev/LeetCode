use std::vec;

// 2273. Find Resultant Array After Removing Anagrams
fn main(){
    let words = ["code","doce","ecod","framer","frame"];
    let words: Vec<String> = words.iter().map(|s| s.to_string()).collect();

    println!("{:?}", Solution::remove_anagrams(words));
}

struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut prev_count = [0; 26];
        for word in words {
            let mut count = [0; 26];
            word.bytes().for_each(|b| count[(b - b'a') as usize] += 1);
            
            if prev_count != count {
                result.push(word);
            }
            prev_count = count;
        }
        result
    }
}