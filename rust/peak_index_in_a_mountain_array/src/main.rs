// Let's call an array A a mountain if the following properties hold:
//     A.length >= 3
//     There exists some 0 < i < A.length - 1 such that
//     A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1]
// Given an array that is definitely a mountain, return any i
// such that A[0] < A[1] < ... A[i-1] < A[i] > A[i+1] > ... > A[A.length - 1].

// Example 1:
// Input: [0,1,0]
// Output: 1

// Example 2:
// Input: [0,2,1,0]
// Output: 1

// Note:
// 3 <= A.length <= 10000
// 0 <= A[i] <= 10^6
// A is a mountain, as defined above.

struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        let len = a.len();
        let mut i = 0;
        while i < len - 1 && a[i] < a[i + 1] {
            i += 1;
        }
        i as i32
    }
}

fn main() {
    let a = vec![0, 1, 0];
    let ac = a.clone();
    println!("{:?} -> {}", ac, Solution::peak_index_in_mountain_array(a));

    let a = vec![0, 2, 1, 0];
    let ac = a.clone();
    println!("{:?} -> {}", ac, Solution::peak_index_in_mountain_array(a));
}
