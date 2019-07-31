// On an infinite plane, a robot initially stands at (0, 0) and faces north.
// The robot can receive one of three instructions:

// "G": go straight 1 unit;
// "L": turn 90 degrees to the left;
// "R": turn 90 degress to the right.

// The robot performs the instructions given in order, and repeats them forever.

// Return true if and only if there exists a circle in the plane
// such that the robot never leaves the circle.

// Example 1:
// Input: "GGLLGG"
// Output: true
// Explanation: The robot moves from (0,0) to (0,2),
// turns 180 degrees, and then returns to (0,0).
// When repeating these instructions,
// the robot remains in the circle of radius 2 centered at the origin.

// Example 2:
// Input: "GG"
// Output: false
// Explanation: The robot moves north indefinitely.

// Example 3:
// Input: "GL"
// Output: true
// Explanation:
// The robot moves from (0, 0) -> (0, 1) -> (-1, 1) -> (-1, 0) -> (0, 0) -> ...

// Note:
// 1 <= instructions.length <= 100
// instructions[i] is in {'G', 'L', 'R'}

struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let dx = [0, 1, 0, -1];
        let dy = [1, 0, -1, 0];
        let mut x = 0;
        let mut y = 0;
        let mut d: usize = 0;
        for c in instructions.chars() {
            if 'G' == c {
                x += dx[d];
                y += dy[d];
            } else if 'L' == c {
                d = (d + 3) % 4;
            } else {
                d = (d + 1) % 4;
            }
        }
        // both of position and direction changed,
        // after at most 4 cycles, it will return to origin
        if (x, y) == (0, 0) || d != 0 {
            return true;
        }
        false
    }
}

fn main() {
    println!("GL -> {}", Solution::is_robot_bounded(String::from("GL")));
    println!("GG -> {}", Solution::is_robot_bounded(String::from("GG")));
}
