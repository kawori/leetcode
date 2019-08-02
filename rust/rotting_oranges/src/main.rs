// In a given grid, each cell can have one of three values:
// the value 0 representing an empty cell;
// the value 1 representing a fresh orange;
// the value 2 representing a rotten orange.

// Every minute, any fresh orange that is adjacent (4-directionally) to a rotten orange becomes rotten.
// Return the minimum number of minutes that must elapse until no cell has a fresh orange.
// If this is impossible, return -1 instead.

// Example 1:
// Input: [[2,1,1],[1,1,0],[0,1,1]]
// Output: 4

// Example 2:
// Input: [[2,1,1],[0,1,1],[1,0,1]]
// Output: -1
// Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten,
// because rotting only happens 4-directionally.

// Example 3:
// Input: [[0,2]]
// Output: 0
// Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.

// Note:
// 1 <= grid.length <= 10
// 1 <= grid[0].length <= 10
// grid[i][j] is only 0, 1, or 2.

struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let nrow = grid.len();
        let ncol = grid[0].len();
        let mut rots: Vec<(usize, usize)> = Vec::new();
        let mut fresh = 0;
        for r in 0..nrow {
            for c in 0..ncol {
                if 2 == grid[r][c] {
                    rots.push((r, c));
                } else if 1 == grid[r][c] {
                    fresh += 1;
                }
            }
        }
        if 0 == fresh {
            return 0;
        }
        if rots.is_empty() {
            return -1;
        }
        let mut ret = 0;
        let mut gc = grid.clone();
        loop {
            let mut tmp: Vec<(usize, usize)> = Vec::new();
            let prev = fresh;
            for p in rots.iter() {
                if p.0 >= 1 && 1 == gc[p.0 - 1][p.1] {
                    tmp.push((p.0 - 1, p.1));
                    gc[p.0 - 1][p.1] = 2;
                    fresh -= 1;
                }
                if p.0 + 1 < nrow && 1 == gc[p.0 + 1][p.1] {
                    tmp.push((p.0 + 1, p.1));
                    gc[p.0 + 1][p.1] = 2;
                    fresh -= 1;
                }
                if p.1 >= 1 && 1 == gc[p.0][p.1 - 1] {
                    tmp.push((p.0, p.1 - 1));
                    gc[p.0][p.1 - 1] = 2;
                    fresh -= 1;
                }
                if p.1 + 1 < ncol && 1 == gc[p.0][p.1 + 1] {
                    tmp.push((p.0, p.1 + 1));
                    gc[p.0][p.1 + 1] = 2;
                    fresh -= 1;
                }
            }
            if fresh < prev {
                ret += 1;
                if 0 == fresh {
                    return ret;
                }
                rots = tmp;
            }
            if fresh == prev {
                return -1;
            }
        }
    }
}

fn main() {
    let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
    println!("{:?} -> {}", grid.clone(), Solution::oranges_rotting(grid));
}
