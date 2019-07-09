// Implement strStr().
// Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:
// Input: haystack = "hello", needle = "ll"
// Output: 2

// Example 2:
// Input: haystack = "aaaaa", needle = "bba"
// Output: -1

// Clarification:
// What should we return when needle is an empty string? This is a great question to ask during an interview.
// For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr() and Java's indexOf().

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let len1 = haystack.len();
        let len2 = needle.len();
        if len1 < len2 {
            return -1;
        }
        let mut i: usize = 0;
        while i <= len1 - len2 {
            if haystack[i..i + len2] == needle {
                return i as i32;
            }
            i += 1;
        }
        return -1;
    }
}

fn main() {
    let haystack = String::from("hello");
    let needle = String::from("ll");
    println!(
        "str_str(\"hello\", \"ll\") = {}",
        Solution::str_str(haystack, needle)
    );
}
