// 49. Group Anagrams
use std::collections::HashMap;

fn main() {
    let strs = ["eat","tea","tan","ate","nat","bat"];
    let strs: Vec<String> = strs.iter().map(|s| s.to_string()).collect();

    println!("{:?}", Solution::group_anagrams(strs));
}

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();

        for s in strs {
            let mut key: Vec<char> = s.chars().collect();
            key.sort();
            map.entry(key).or_insert(vec![]).push(s);
        }

        map.values().cloned().collect()
    }
}