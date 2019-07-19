// The Hamming distance between two integers is the number of positions at which the corresponding bits are different.
// Given two integers x and y, calculate the Hamming distance.

// Note:
// 0 ≤ x, y < 231.

// Example:
// Input: x = 1, y = 4
// Output: 2
// Explanation:
// 1   (0 0 0 1)
// 4   (0 1 0 0)
//        ↑   ↑
// The above arrows point to positions where the corresponding bits are different.


struct Solution;

impl Solution {
    pub fn hamming_distance2(x: i32, y: i32) -> i32 {
        let mut z = x ^ y;
        let mut ret = 0;
        while z != 0 {
            if z % 2 != 0 {
                ret += 1;
            }
            z = z >> 1;
        }
        ret
    }
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        format!("{:b}", x ^ y).chars().filter(|c| *c == '1').count() as i32
    }
}

fn main() {
    println!("{}, {} -> {}", 1, 4, Solution::hamming_distance(1, 4));
    println!("{}, {} -> {}", 1, 4, Solution::hamming_distance2(1, 4));
}
