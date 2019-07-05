// Given a non-negative?index k?where k ??33,
// return the kth?index row of the Pascal's triangle.
// Note that the row index starts from?0.

// Example:
// Input: 3
// Output: [1,3,3,1]

// Follow up:
// Could you optimize your algorithm to use only O(k) extra space?

struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if 0 == row_index {
            return vec![1];
        } else if 1 == row_index {
            return vec![1, 1];
        } else {
            let pre = Solution::get_row(row_index - 1);
            let mut ret: Vec<i32> = Vec::new();
            ret.push(1);
            let pre_len = pre.len() - 1;
            let mut i: usize = 0;
            while i < pre_len {
                ret.push(pre[i] + pre[i+1]);
                i += 1;
            }
            ret.push(1);
            ret
        }
    }
}

fn main() {
    let row = Solution::get_row(3);
    println!("{:?}", row);
}
