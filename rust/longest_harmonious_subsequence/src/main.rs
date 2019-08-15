// We define a harmounious array as an array where the difference between its maximum value and its minimum value is exactly 1.
// Now, given an integer array, you need to find the length of its longest harmonious subsequence among all its possible subsequences.

// Example 1:
// Input: [1,3,2,2,5,2,3,7]
// Output: 5
// Explanation: The longest harmonious subsequence is [3,2,2,2,3].

// Note: The length of the input array will not exceed 20,000.

use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        if 2 > len {
            return 0;
        }
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        for n in nums.iter() {
            if let Some(e) = map.get_mut(n) {
                *e += 1;
            } else {
                map.insert(*n, 1);
            }
        }
        if 1 == map.len() {
            return 0;
        }
        if let Some(x) = map
            .iter()
            .zip(map.iter().skip(1))
            .filter(|(x, y)| y.0 - x.0 == 1)
            .map(|(x, y)| x.1 + y.1)
            .max()
        {
            return x;
        } else {
            return 0;
        }
    }
}

fn main() {
    let nums = vec![1, 3, 2, 2, 5, 2, 3, 7];
    let nc = nums.clone();
    println!("{:?} -> {}", nc, Solution::find_lhs(nums));
}
