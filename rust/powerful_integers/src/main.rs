// Given two positive integers x and y,
// an integer is powerful if it is equal to x^i + y^j for some integers i >= 0 and j >= 0.
// Return a list of all powerful integers that have value less than or equal to bound.
// You may return the answer in any order. In your answer, each value should occur at most once.

// Example 1:
// Input: x = 2, y = 3, bound = 10
// Output: [2,3,4,5,7,9,10]
// Explanation:
// 2 = 2^0 + 3^0
// 3 = 2^1 + 3^0
// 4 = 2^0 + 3^1
// 5 = 2^1 + 3^1
// 7 = 2^2 + 3^1
// 9 = 2^3 + 3^0
// 10 = 2^0 + 3^2

// Example 2:
// Input: x = 3, y = 5, bound = 15
// Output: [2,4,6,8,10,14]

// Note:
// 1 <= x <= 100
// 1 <= y <= 100
// 0 <= bound <= 10^6

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        if 2 > bound {
            return Vec::new();
        }
        if 1 == x && 1 == y {
            return vec![2];
        }
        let mut set: HashSet<i32> = HashSet::new();
        if 1 == x || 1 == y {
            let z = std::cmp::max(x, y);
            let mut j = 0;
            let mut n = 1 + z.pow(j);
            while n <= bound {
                set.insert(n);
                j += 1;
                n = 1 + z.pow(j);
            }
        } else {
            let mut i = 0;
            loop {
                let mut j = 0;
                let l = x.pow(i);
                let mut n = l + y.pow(j);
                while n <= bound {
                    set.insert(n);
                    j += 1;
                    n = l + y.pow(j);
                }
                if 0 == j {
                    break;
                }
                i += 1;
            }
        }
        let ret: Vec<i32> = set.into_iter().collect();
        ret
    }
}

fn main() {
    let x = 2;
    let y = 3;
    let bound = 10;
    println!(
        "x: {}, y: {}, bound: {} -> {:?}",
        x,
        y,
        bound,
        Solution::powerful_integers(x, y, bound)
    );
}
