// Given a string containing just the characters '(', ')', '{', '}', '[' and ']'
// determine if the input string is valid.

// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Note that an empty string is also considered valid.

// Example 1:
// Input: "()"
// Output: true

// Example 2:
// Input: "()[]{}"
// Output: true

// Example 3:
// Input: "(]"
// Output: false

// Example 4:
// Input: "([)]"
// Output: false

// Example 5:
// Input: "{[]}"
// Output: true

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut stack: Vec<u8> = Vec::new();
        for ch in s.as_bytes() {
            if b'(' == *ch || b'[' == *ch || b'{' == *ch {
                stack.push(*ch);
            } else if b')' == *ch {
                let p = stack.pop();
                if p.is_none() {
                    return false;
                } else if b'(' != p.unwrap() {
                    return false;
                }
            } else if b']' == *ch {
                let p = stack.pop();
                if p.is_none() {
                    return false;
                } else if b'[' != p.unwrap() {
                    return false;
                }
            } else {
                let p = stack.pop();
                if p.is_none() {
                    return false;
                } else if b'{' != p.unwrap() {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}

fn main() {
    println!("() -> {}", Solution::is_valid(String::from("()")));
    println!("()[]{{}} -> {}", Solution::is_valid(String::from("()[]{}")));
    println!("(] -> {}", Solution::is_valid(String::from("(]")));
    println!("([)] -> {}", Solution::is_valid(String::from("([)]")));
    println!("{{[]}} -> {}", Solution::is_valid(String::from("{[]}")));
}
