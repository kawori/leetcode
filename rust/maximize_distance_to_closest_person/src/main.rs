// In a row of seats,
// 1 represents a person sitting in that seat,
// and 0 represents that the seat is empty.
// There is at least one empty seat, and at least one person sitting.
// Alex wants to sit in the seat such that the distance between him
// and the closest person to him is maximized.
// Return that maximum distance to closest person.

// Example 1:
// Input: [1,0,0,0,1,0,1]
// Output: 2
// Explanation:
// If Alex sits in the second open seat (seats[2]),
// then the closest person has distance 2.
// If Alex sits in any other open seat, the closest person has distance 1.
// Thus, the maximum distance to the closest person is 2.

// Example 2:
// Input: [1,0,0,0]
// Output: 3
// Explanation:
// If Alex sits in the last seat, the closest person is 3 seats away.
// This is the maximum distance possible, so the answer is 3.

// Note:
// 1 <= seats.length <= 20000
// seats contains only 0s or 1s, at least one 0, and at least one 1.

struct Solution;

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut i = 0;
        let len = seats.len();
        let mut max = 1;
        while i < len {
            if 0 == seats[i] {
                if 0 == i || 1 == seats[i - 1] {
                    let beg = i;
                    while i < len && 0 == seats[i] {
                        i += 1;
                    }
                    let cur = if 0 == beg {
                        i
                    } else {
                        let zero_cnt = i - beg;
                        if len == i {
                            zero_cnt
                        } else {
                            if 0 != zero_cnt % 2 {
                                zero_cnt / 2 + 1
                            } else {
                                zero_cnt / 2
                            }
                        }
                    };
                    if cur > max {
                        max = cur;
                    }
                }
            } else {
                i += 1;
            }
        }
        max as i32
    }
}

fn main() {
    let seats = vec![1, 0, 0, 0, 1, 0, 1];
    println!(
        "{:?} -> {}",
        seats.clone(),
        Solution::max_dist_to_closest(seats)
    );
    let seats = vec![1, 0, 0, 0];
    println!(
        "{:?} -> {}",
        seats.clone(),
        Solution::max_dist_to_closest(seats)
    );
}
