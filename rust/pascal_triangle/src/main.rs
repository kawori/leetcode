// Given a non-negative integer numRows, generate the first numRows of Pascal's triangle.

// Example:

// Input: 5
// Output:
// [
//      [1],
//     [1,1],
//    [1,2,1],
//   [1,3,3,1],
//  [1,4,6,4,1]
// ]

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();
        if num_rows > 0 {
            ret.push(vec![1]);
        }
        if num_rows > 1 {
            ret.push(vec![1, 1]);
            let mut i: usize = 2;
            while i < (num_rows as usize) {
                let mut row: Vec<i32> = vec![1; i + 1];
                let mut j: usize = 1;
                while j < i {
                    row[j] = ret[i - 1][j - 1] + ret[i - 1][j];
                    j += 1;
                }
                ret.push(row);
                i += 1;
            }
        }
        ret
    }
}

fn main() {
    let tri = Solution::generate(5);
    println!("5:\n{:?}", tri);
}
