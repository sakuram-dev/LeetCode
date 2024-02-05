// 876. Middle of the Linked List

use std::collections::HashMap;
fn main() {
    let ranson_note = "aa".to_string();
    let magazine = "abcabc".to_string();
    println!("{}", Solution::can_construct(ranson_note, magazine));
}

struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let length_ranson_note = ransom_note.chars().count();
        let length_magazine = magazine.chars().count();
        if length_ranson_note > length_magazine {
            return false;
        }

        let mut map = HashMap::new();

        for c in magazine.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in ransom_note.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;

            if *count < 0 {
                return false;
            }
        }

        true
    }
}