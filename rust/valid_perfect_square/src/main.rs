// Given a positive integer num, write a function which returns True if num is a perfect square else False.
// Note: Do not use any built-in library function such as sqrt.

// Example 1:
// Input: 16
// Output: true

// Example 2:
// Input: 14
// Output: false

struct Solution;

impl Solution {
    // binary search
    pub fn is_perfect_square(num: i32) -> bool {
        let r = num % 10;
        if 2 == r || 3 == r || 7 == r || 8 == r {
            return false;
        }
        let mut l = 1;
        let mut h = num as u64; // avoid overflow
        while l <= h {
            let m = (l + h) / 2;
            let s = m * m;
            if s == num as u64 {
                return true;
            } else if s < num as u64 {
                l = m + 1;
            } else {
                h = m - 1;
            }
        }
        false
    }

    // 1+3+5+...+(2n-1)=n^2
    pub fn is_perfect_square2(num: i32) -> bool {
        let mut i = 1;
        let mut n = num;
        while n > 0 {
            n -= i;
            i += 2;
        }
        n == 0
    }
}

fn main() {
    let num = 808201;
    println!("{} -> {}", num, Solution::is_perfect_square2(num));
}
