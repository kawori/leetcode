// Given an array consisting of n integers,
// find the contiguous subarray of given length k that has the maximum average value.
// And you need to output the maximum average value.

// Example 1:
// Input: [1,12,-5,-6,50,3], k = 4
// Output: 12.75
// Explanation: Maximum average is (12-5-6+50)/4 = 51/4 = 12.75

// Note:
// 1 <= k <= n <= 30,000.
// Elements of the given array will be in the range [-10,000, 10,000].

struct Solution;

impl Solution {
    // sliding window method
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mk = k as usize;
        let mut max: i32 = nums.iter().take(mk).sum();
        let mut last = max;
        for i in 1..nums.len() - mk + 1 {
            last += nums[i + mk - 1] - nums[i - 1];
            if last > max {
                max = last;
            }
        }
        (max as f64) / (k as f64)
    }

    // dynamic planning method
    pub fn find_max_average2(nums: Vec<i32>, k: i32) -> f64 {
        let len = nums.len();
        let mut sums = vec![0; len + 1];
        sums[0] = 0;
        let mut max = std::i32::MIN;
        let mk = k as usize;
        for i in 1..len + 1 {
            sums[i] = sums[i - 1] + nums[i - 1];
            if i >= mk {
                let cur = sums[i] - sums[i - mk];
                if cur > max {
                    max = cur;
                }
            }
        }
        (max as f64) / (k as f64)
    }
}

fn main() {
    let nums = vec![1, 12, -5, -6, 50, 3];
    let k = 4;
    println!(
        "{:?}, {} -> {}",
        nums.clone(),
        k,
        Solution::find_max_average(nums, k)
    );
}
