// Given two strings s and t, determine if they are isomorphic.
// Two strings are isomorphic if the characters in s can be replaced to get t.
// All occurrences of a character must be replaced with another character
// while preserving the order of characters.
// No two characters may map to the same character but a character may map to itself.

// Example 1:
// Input: s = "egg", t = "add"
// Output: true

// Example 2:
// Input: s = "foo", t = "bar"
// Output: false

// Example 3:
// Input: s = "paper", t = "title"
// Output: true

// Note:
// You may assume both s and t have the same length.

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        for (l, r) in s.chars().zip(t.chars()) {
            if s.find(l) != t.find(r) {
                return false;
            }
        }
        true
    }
}

fn main() {
    let s = String::from("paper");
    let t = String::from("title");
    println!(
        "\"{}\", \"{}\" -> {}",
        s.clone(),
        t.clone(),
        Solution::is_isomorphic(s, t)
    );
}
