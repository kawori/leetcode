// Given an integer array, you need to find one continuous subarray that
// if you only sort this subarray in ascending order,
// then the whole array will be sorted in ascending order, too.

// You need to find the shortest such subarray and output its length.

// Example 1:
// Input: [2, 6, 4, 8, 10, 9, 15]
// Output: 5
// Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to
// make the whole array sorted in ascending order.

// Note:
// Then length of the input array is in range [1, 10,000].
// The input array may contain duplicates, so ascending order here means <=.

struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut left = 0;
        for l in 0..len - 1 {
            if nums[l] > nums[l + 1] {
                left = l;
                break;
            }
        }
        let mut right = 0;
        for r in (1..len).rev() {
            if nums[r] < nums[r - 1] {
                right = r;
                break;
            }
        }
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;
        for i in left..right + 1 {
            if nums[i] < min {
                min = nums[i];
            }
            if nums[i] > max {
                max = nums[i];
            }
        }
        for i in (0..left).rev() {
            if nums[i] <= min {
                break;
            }
            left = i;
        }
        for i in right + 1..len {
            if nums[i] >= max {
                break;
            }
            right = i;
        }
        if left == right {
            return 0;
        }
        (right - left + 1) as i32
    }
}

fn main() {
    let arr = vec![2, 6, 4, 8, 10, 9, 15];
    println!("{:?} -> {}", arr.clone(), Solution::find_unsorted_subarray(arr));
}
