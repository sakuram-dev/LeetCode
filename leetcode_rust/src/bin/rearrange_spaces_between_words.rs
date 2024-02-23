// 1592. Rearrange Spaces Between Words

fn main() {
    let text = "  this   is  a sentence ";
    println!("{}", Solution::reorder_spaces(text.to_string()));
}

struct Solution;

impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        // count spaces
        let mut spaces = 0;
        for c in text.chars() {
            if c == ' ' {
                spaces += 1;
            }
        }

        // split words
        let words: Vec<&str> = text.split_whitespace().collect();
        let words_count = words.len();

        // rearrange spaces
        if words_count == 1 {
            return words[0].to_string() + &" ".repeat(spaces);
        }
        let space_between_words = spaces / (words_count - 1);
        let mut result = String::new();
        for i in 0..words_count {
            result += words[i];
            if i != words_count - 1 {
                result += &" ".repeat(space_between_words);
            }
        }
        result += &" ".repeat(spaces % (words_count - 1));
        result
    }
}