// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

// Example:

// Input: [-2,1,-3,4,-1,2,1,-5,4],
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
// Follow up:

// If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut ret = std::i32::MIN;
        let mut tmp = -1;
        for n in nums {
            tmp = std::cmp::max(tmp + n, n);
            ret = std::cmp::max(tmp, ret);
        }
        ret
    }
}

fn main() {
    let l: Vec<i32> = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let lc = l.clone();
    let ret = Solution::max_sub_array(l);
    println!("{:?} -> {}", lc, ret);
}
