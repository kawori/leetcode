// Given a pattern and a string str, find if str follows the same pattern.
// Here follow means a full match, such that
// there is a bijection between a letter in pattern and a non-empty word in str.

// Example 1:
// Input: pattern = "abba", str = "dog cat cat dog"
// Output: true

// Example 2:
// Input:pattern = "abba", str = "dog cat cat fish"
// Output: false

// Example 3:
// Input: pattern = "aaaa", str = "dog cat cat dog"
// Output: false

// Example 4:
// Input: pattern = "abba", str = "dog dog dog dog"
// Output: false

// Notes:
// You may assume pattern contains only lowercase letters,
// and str contains lowercase letters that may be separated by a single space.

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, str: String) -> bool {
        let mut map1: HashMap<char, Vec<usize>> = HashMap::new();
        for (i, c) in pattern.chars().enumerate() {
            if let Some(x) = map1.get_mut(&c) {
                x.push(i);
            } else {
                map1.insert(c, vec![i]);
            }
        }
        let mut map2: HashMap<&str, Vec<usize>> = HashMap::new();
        for (i, s) in str.split(' ').enumerate() {
            if let Some(x) = map2.get_mut(s) {
                x.push(i);
            } else {
                map2.insert(s, vec![i]);
            }
        }
        let mut v1: Vec<&Vec<usize>> = map1.values().collect();
        let mut v2: Vec<&Vec<usize>> = map2.values().collect();
        v1.sort();
        v2.sort();
        v1 == v2
    }
}

fn main() {
    let pattern = String::from("abba");
    let s = String::from("cat dog dog fish");
    println!(
        "\"abba\", \"cat dog dog fish\" -> {}",
        Solution::word_pattern(pattern, s)
    );
}
