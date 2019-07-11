// Given a string, sort it in decreasing order based on the frequency of characters.

// Example 1:
// Input:
// "tree"
// Output:
// "eert"
// Explanation:
// 'e' appears twice while 'r' and 't' both appear once.
// So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.

// Example 2:
// Input:
// "cccaaa"
// Output:
// "cccaaa"
// Explanation:
// Both 'c' and 'a' appear three times, so "aaaccc" is also a valid answer.
// Note that "cacaca" is incorrect, as the same characters must be together.

// Example 3:
// Input:
// "Aabb"
// Output:
// "bbAa"
// Explanation:
// "bbaA" is also a valid answer, but "Aabb" is incorrect.
// Note that 'A' and 'a' are treated as two different characters.

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut freq_map = HashMap::new();
        for ch in s.bytes() {
            if let Some(f) = freq_map.get_mut(&ch) {
                *f += 1;
            } else {
                freq_map.insert(ch, 1);
            }
        }
        let mut freq_vec: Vec<(&u8, &i32)> = freq_map.iter().collect();
        freq_vec.sort_by(|a, b| b.1.cmp(a.1));
        let mut ret = String::from("");
        for elem in freq_vec {
            let s = String::from_utf8(vec![*elem.0;*elem.1 as usize]).unwrap();
            ret.push_str(&s);
        }
        ret
    }
}

fn main() {
    let s = "Aabb".to_string();
    let sc = s.clone();
    println!("{} -> {}", sc, Solution::frequency_sort(s));
}
