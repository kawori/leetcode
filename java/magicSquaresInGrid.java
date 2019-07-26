// A 3 x 3 magic square is a 3 x 3 grid filled with distinct numbers from 1 to 9
// such that each row, column, and both diagonals all have the same sum.

// Given an grid of integers, how many 3 x 3 "magic square" subgrids are there?
// (Each subgrid is contiguous).

// Example 1:
// Input: [[4,3,8,4],
//         [9,5,1,9],
//         [2,7,6,2]]
// Output: 1
// Explanation:
// The following subgrid is a 3 x 3 magic square:
// 438
// 951
// 276

// while this one is not:
// 384
// 519
// 762

// In total, there is only one magic square inside the given grid.

// Note:
// 1 <= grid.length <= 10
// 1 <= grid[0].length <= 10
// 0 <= grid[i][j] <= 15

import java.util.HashSet;

class magicSquaresInGrid {
    public boolean isMagic(int[][] grid, int r, int c) {
        HashSet<Integer> set = new HashSet<Integer>();
        for (int i = 0; i < 3; ++i) {
            for (int j = 0; j < 3; ++j) {
                int elem = grid[r + i][c + j];
                if (set.contains(elem) || elem < 1 || elem > 9) {
                    return false;
                }
                set.add(elem);
            }
        }
        if (15 != grid[r][c] + grid[r + 1][c + 1] + grid[r + 2][c + 2]) {
            return false;
        }
        if (15 != grid[r + 2][c] + grid[r + 1][c + 1] + grid[r][c + 2]) {
            return false;
        }
        for (int i = 0; i < 3; ++i) {
            int s = 0;
            for (int j = 0; j < 3; ++j) {
                s += grid[r + i][c + j];
            }
            if (15 != s) {
                return false;
            }
        }
        for (int i = 0; i < 3; ++i) {
            int s = 0;
            for (int j = 0; j < 3; ++j) {
                s += grid[r + j][c + i];
            }
            if (15 != s) {
                return false;
            }
        }
        return true;
    }

    public int numMagicSquaresInside(int[][] grid) {
        int nrow = grid.length, ncol = grid[0].length;
        if (nrow < 3 || ncol < 3) {
            return 0;
        }
        int ret = 0;
        for (int r = 0; r < nrow - 2; ++r) {
            for (int c = 0; c < ncol - 2; ++c) {
                if (5 != grid[r][c] && 5 == grid[r + 1][c + 1] && isMagic(grid, r, c)) {
                    ret++;
                }
            }
        }
        return ret;
    }
}