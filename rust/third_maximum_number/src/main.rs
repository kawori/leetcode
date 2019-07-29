// Given a non-empty array of integers,
// return the third maximum number in this array.
// If it does not exist, return the maximum number.
// The time complexity must be in O(n).

// Example 1:
// Input: [3, 2, 1]
// Output: 1
// Explanation: The third maximum is 1.

// Example 2:
// Input: [1, 2]
// Output: 2
// Explanation:
// The third maximum does not exist, so the maximum (2) is returned instead.

// Example 3:
// Input: [2, 2, 3, 1]
// Output: 1
// Explanation:
// Note that the third maximum here means the third maximum distinct number.
// Both numbers with value 2 are both considered as second maximum.

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    // min-heap solution
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let l = set.len();
        if l < 3 {
            return set.into_iter().max().unwrap();
        }
        let mut heap = BinaryHeap::new();
        for elem in set.iter().take(3) {
            heap.push(Reverse(*elem));
        }
        for elem in set.iter().skip(3) {
            if let Some(Reverse(n)) = heap.peek() {
                if *elem > *n {
                    heap.pop();
                    heap.push(Reverse(*elem));
                }
            }
        }
        let Reverse(n) = heap.peek().unwrap();
        *n
    }
}

fn main() {
    println!("[2, 2, 3, 1] -> {}", Solution::third_max(vec![2, 2, 3, 1]));
}
