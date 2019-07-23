// On an 8 x 8 chessboard, there is one white rook. 
// There also may be empty squares, white bishops, and black pawns.
// These are given as characters 'R', '.', 'B', and 'p' respectively.
// Uppercase characters represent white pieces,
// and lowercase characters represent black pieces.
// The rook moves as in the rules of Chess:
// it chooses one of four cardinal directions (north, east, west, and south),
// then moves in that direction until it chooses to stop,
// reaches the edge of the board,
// or captures an opposite colored pawn by moving to the same square it occupies.
// Also, rooks cannot move into the same square as other friendly bishops.
// Return the number of pawns the rook can capture in one move.

// Note:
// board.length == board[i].length == 8
// board[i][j] is either 'R', '.', 'B', or 'p'
// There is exactly one cell with board[i][j] == 'R'

// Example 1:
// Input:
// [[".",".",".",".",".",".",".","."],
//  [".",".",".","p",".",".",".","."],
//  [".",".",".","R",".",".",".","p"],
//  [".",".",".",".",".",".",".","."],
//  [".",".",".",".",".",".",".","."],
//  [".",".",".","p",".",".",".","."],
//  [".",".",".",".",".",".",".","."],
//  [".",".",".",".",".",".",".","."]]
// Output: 3
// Explanation:
// In this example the rook is able to capture all the pawns.

// Example 2:
// Input:
// [[".",".",".",".",".",".",".","."],
//  [".","p","p","p","p","p",".","."],
//  [".","p","p","B","p","p",".","."],
//  [".","p","B","R","B","p",".","."],
//  [".","p","p","B","p","p",".","."],
//  [".","p","p","p","p","p",".","."],
//  [".",".",".",".",".",".",".","."],
//  [".",".",".",".",".",".",".","."]]
// Output: 0
// Explanation:
// Bishops are blocking the rook to capture any pawn.

// Input:
// [[".",".",".",".",".",".",".","."],
//  [".",".",".","p",".",".",".","."],
//  [".",".",".","p",".",".",".","."],
//  ["p","p",".","R",".","p","B","."],
//  [".",".",".",".",".",".",".","."],
//  [".",".",".","B",".",".",".","."],
//  [".",".",".","p",".",".",".","."],
//  [".",".",".",".",".",".",".","."]]
// Output: 3
// Explanation:
// The rook can capture the pawns at positions b5, d6 and f5.

class Solution {
    public int[] locate(char[][] board) {
        int[] pos = { 0, 0 };
        for (int i = 0; i < 8; ++i) {
            for (int j = 0; j < 8; ++j) {
                if ('R' == board[i][j]) {
                    pos[0] = i;
                    pos[1] = j;
                    return pos;
                }
            }
        }
        return pos;
    }

    public int numRookCaptures(char[][] board) {
        int[] pos = locate(board);
        int rr = pos[0], rc = pos[1];
        int ret = 0;
        for (int i = rc; i >= 0; --i) {
            if ('p' == board[rr][i]) {
                ret++;
                break;
            } else if ('B' == board[rr][i]) {
                break;
            }
        }
        for (int i = rc; i < 8; ++i) {
            if ('p' == board[rr][i]) {
                ret++;
                break;
            } else if ('B' == board[rr][i]) {
                break;
            }
        }
        for (int i = rr; i >= 0; --i) {
            if ('p' == board[i][rc]) {
                ret++;
                break;
            } else if ('B' == board[i][rc]) {
                break;
            }
        }
        for (int i = rr; i < 8; ++i) {
            if ('p' == board[i][rc]) {
                ret++;
                break;
            } else if ('B' == board[i][rc]) {
                break;
            }
        }
        return ret;
    }
}
