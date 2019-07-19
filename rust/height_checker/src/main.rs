// Students are asked to stand in non-decreasing order of heights for an annual photo.
// Return the minimum number of students not standing in the right positions.
// (This is the number of students that must move in order for all students to be standing in non-decreasing order of height.)

// Example 1:
// Input: [1,1,4,2,1,3]
// Output: 3
// Explanation:
// Students with heights 4, 3 and the last 1 are not standing in the right positions.

// Note:
// 1 <= heights.length <= 100
// 1 <= heights[i] <= 100

struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut ret = heights.clone();
        ret.sort();
        ret.iter().zip(heights.iter()).filter(|x| x.0 != x.1).count() as i32
    }
}

fn main() {
    let heights = vec![1,1,4,2,1,3];
    let hc = heights.clone();
    println!("{:?} -> {}", hc, Solution::height_checker(heights));
}
