// Given an array of integers and an integer k,
// you need to find the number of unique k-diff pairs in the array.
// Here a k-diff pair is defined as an integer pair (i, j),
// where i and j are both numbers in the array and their absolute difference is k.

// Example 1:
// Input: [3, 1, 4, 1, 5], k = 2
// Output: 2
// Explanation: There are two 2-diff pairs in the array, (1, 3) and (3, 5).
// Although we have two 1s in the input, we should only return the number of unique pairs.

// Example 2:
// Input:[1, 2, 3, 4, 5], k = 1
// Output: 4
// Explanation: There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).

// Example 3:
// Input: [1, 3, 1, 5, 4], k = 0
// Output: 1
// Explanation: There is one 0-diff pair in the array, (1, 1).

// Note:
// The pairs (i, j) and (j, i) count as the same pair.
// The length of the array won't exceed 10,000.
// All the integers in the given input belong to the range: [-1e7, 1e7].

use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        if 2 > len || k < 0 {
            return 0;
        }
        // BTreeMap is sorted by keys
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        for n in nums {
            if let Some(x) = map.get_mut(&n) {
                *x += 1;
            } else {
                map.insert(n, 1);
            }
        }
        println!("{:?}", map);
        let mut ret = 0;
        for key in map.keys() {
            if 0 == k {
                if map[key] >= 2 {
                    ret += 1;
                }
            } else {
                let con = key + k;
                if map.contains_key(&con) {
                    ret += 1;
                }
            }
        }
        ret
    }
}

fn main() {
    println!("{}", Solution::find_pairs(vec![3, 1, 4, 1, 5], 1));
}
