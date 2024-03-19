// 2085. Count Common Words With One Occurrence
fn main() {
    let text1: Vec<String> = vec!["this".to_string(), "apple".to_string(), "is".to_string(), "sweet".to_string()];
    let text2: Vec<String> = vec!["this".to_string(), "apple".to_string(), "is".to_string()];

    let result = Solution::count_words(text1, text2);

    println!("result = {}", result);
}

struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;

        // Create HashMaps for both word lists
        let mut word_count1 = HashMap::new();
        let mut word_count2 = HashMap::new();

        // Count occurrences of each word in the first list
        for word in &words1 {
            let count = word_count1.entry(word).or_insert(0);
            *count += 1;
        }

        // For each word in the second list, if it occurs only once in the first list,
        // increment its count in the second HashMap
        for word in &words2 {
            if let Some(count) = word_count1.get(word) {
                if *count == 1 {
                    let count_in_second = word_count2.entry(word).or_insert(0);
                    *count_in_second += 1;
                }
            }
        }

        // Count the words that occur only once in both lists
        let common_single_occurrence_words = word_count2.values().filter(|&v| *v == 1).count();

        common_single_occurrence_words as i32
    }
}