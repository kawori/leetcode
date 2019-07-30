// Three stones are on a number line at positions a, b, and c.

// Each turn, you pick up a stone at an endpoint
// (ie., either the lowest or highest position stone),
// and move it to an unoccupied position between those endpoints.
// Formally, let's say the stones are currently at positions x, y, z with x < y < z.
// You pick up the stone at either position x or position z,
// and move that stone to an integer position k, with x < k < z and k != y.

// The game ends when you cannot make any more moves,
// ie. the stones are in consecutive positions.

// When the game ends, what is the minimum and maximum number of moves
// that you could have made?
// Return the answer as an length 2 array: answer = [minimum_moves, maximum_moves]

// Example 1:
// Input: a = 1, b = 2, c = 5
// Output: [1,2]
// Explanation: Move the stone from 5 to 3, or move the stone from 5 to 4 to 3.

// Example 2:
// Input: a = 4, b = 3, c = 2
// Output: [0,0]
// Explanation: We cannot make any moves.

// Example 3:
// Input: a = 3, b = 5, c = 1
// Output: [1,2]
// Explanation: Move the stone from 1 to 4; or move the stone from 1 to 2 to 4.

// Note:
// 1 <= a <= 100
// 1 <= b <= 100
// 1 <= c <= 100
// a != b, b != c, c != a

struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut stones = vec![a, b, c];
        stones.sort();
        let (x, y, z) = (stones[0], stones[1], stones[2]);
        let mut ret = vec![2, z - x - 2];
        if x + 1 == y {
            ret[0] -= 1;
        }
        if y + 1 == z {
            ret[0] -= 1;
        }
        if x + 2 == y || y + 2 == z {
            ret[0] = 1;
        }
        ret
    }
}

fn main() {
    println!("{}, {}, {} -> {:?}", 1, 2, 5, Solution::num_moves_stones(1, 2, 5));
    println!("{}, {}, {} -> {:?}", 4, 3, 2, Solution::num_moves_stones(4, 3, 2));
    println!("{}, {}, {} -> {:?}", 3, 5, 1, Solution::num_moves_stones(3, 5, 1));
}
