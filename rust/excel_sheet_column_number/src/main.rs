// Given a column title as appear in an Excel sheet,
// return its corresponding column number.

// For example:
// A -> 1
// B -> 2
// C -> 3
// ...
// Z -> 26
// AA -> 27
// AB -> 28
// ...

// Example 1:
// Input: "A"
// Output: 1

// Example 2:
// Input: "AB"
// Output: 28

// Example 3:
// Input: "ZY"
// Output: 701

struct Solution;

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut ret: i32 = 0;
        let cnt = s.len() as u32;
        let base: i32 = 26;
        for (i, c) in s.chars().enumerate() {
            ret += base.pow(cnt - i as u32 - 1) * (c as i32 - 'A' as i32 + 1);
        }
        ret
    }
}

fn main() {
    println!("A  -> {}", Solution::title_to_number("A".to_string()));
    println!("AB -> {}", Solution::title_to_number("AB".to_string()));
    println!("ZY -> {}", Solution::title_to_number("ZY".to_string()));
}
