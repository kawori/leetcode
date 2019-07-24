// Given a non-empty array of integers,
// every element appears twice except for one.
// Find that single one.

// Note:
// Your algorithm should have a linear runtime complexity.
// Could you implement it without using extra memory?

// Example 1:
// Input: [2,2,1]
// Output: 1

// Example 2:
// Input: [4,1,2,1,2]
// Output: 4

// a ^ b ^ a = a ^ a ^ b = 0 ^ b = b

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, x| acc ^ x)
    }
}

fn main() {
    let numbers = vec![4, 1, 2, 1, 2];
    println!(
        "{:?} -> {}",
        numbers.clone(),
        Solution::single_number(numbers)
    );
}
