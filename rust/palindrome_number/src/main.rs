// Determine whether an integer is a palindrome.
// An integer is a palindrome when it reads the same backward as forward.

// Example 1:
// Input: 121
// Output: true

// Example 2:
// Input: -121
// Output: false
// Explanation: From left to right, it reads -121.
// From right to left, it becomes 121-. Therefore it is not a palindrome.

// Example 3:
// Input: 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// Follow up:
// Coud you solve it without converting the integer to a string?

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        } else {
            let mut digits: Vec<i32> = Vec::new();
            let mut y = x;
            while y != 0 {
                digits.push(y % 10);
                y /= 10;
            }
            let len = digits.len();
            let mut i: usize = 0;
            while i < len / 2 {
                if digits[i] != digits[len - 1 - i] {
                    return false;
                }
                i += 1;
            }
            true
        }
    }
}

fn main() {
    println!("121: {}", Solution::is_palindrome(121));
    println!("-121: {}", Solution::is_palindrome(-121));
    println!("10: {}", Solution::is_palindrome(10));
}
