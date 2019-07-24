// Given two strings A and B of lowercase letters,
// return true if and only ifwe can swap two letters in A so that the result equals B.

// Example 1:
// Input: A = "ab", B = "ba"
// Output: true

// Example 2:
// Input: A = "ab", B = "ab"
// Output: false

// Example 3:
// Input: A = "aa", B = "aa"
// Output: true

// Example 4:
// Input: A = "aaaaaaabc", B = "aaaaaaacb"
// Output: true

// Example 5:
// Input: A = "", B = "aa"
// Output: false

// Note:
// 0 <= A.length <= 20000
// 0 <= B.length <= 20000
// A and B consist only of lowercase letters.

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn has_duplicates(s: &str) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        for ch in s.chars() {
            if set.contains(&ch) {
                return true;
            }
            set.insert(ch);
        }
        false
    }
    pub fn buddy_strings(a: String, b: String) -> bool {
        let lena = a.len();
        let lenb = b.len();
        if lena < 2 || lena != lenb {
            return false;
        }
        if a == b && !Solution::has_duplicates(&a) {
            return false;
        }
        if a != b {
            let diff_pos: Vec<usize> = a
                .chars()
                .zip(b.chars())
                .enumerate()
                .filter(|(_, (c1, c2))| c1 != c2)
                .map(|(i, _)| i)
                .collect();
            if diff_pos.len() != 2 {
                return false;
            }
            let ab = a.as_bytes();
            let bb = b.as_bytes();
            if ab[diff_pos[0]] != bb[diff_pos[1]] || ab[diff_pos[1]] != bb[diff_pos[0]] {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "\"ab\", \"ba\" -> {}",
        Solution::buddy_strings(String::from("ab"), String::from("ba"))
    );
    println!(
        "\"ab\", \"ab\" -> {}",
        Solution::buddy_strings(String::from("ab"), String::from("ab"))
    );
    println!(
        "\"aa\", \"aa\" -> {}",
        Solution::buddy_strings(String::from("aa"), String::from("aa"))
    );
    println!(
        "\"aaaaaaabc\", \"aaaaaaacb\" -> {}",
        Solution::buddy_strings(String::from("aaaaaaabc"), String::from("aaaaaaacb"))
    );
    println!(
        "\"\", \"aa\" -> {}",
        Solution::buddy_strings(String::from(""), String::from("aa"))
    );
}
