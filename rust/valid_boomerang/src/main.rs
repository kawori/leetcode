// A boomerang is a set of 3 points that are all distinct and not in a straight line.
// Given a list of three points in the plane, return whether these points are a boomerang.

// Example 1:
// Input: [[1,1],[2,3],[3,2]]
// Output: true

// Example 2:
// Input: [[1,1],[2,2],[3,3]]
// Output: false

// Note:
// points.length == 3
// points[i].length == 2
// 0 <= points[i][j] <= 100

struct Solution;

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let pa = &points[0];
        let pb = &points[1];
        let pc = &points[2];
        if pa == pb || pa == pc || pb == pc {
            return false;
        }
        if pa[0] == pb[0] && pa[0] == pc[0] {
            return false;
        }
        if pa[0] != pb[0] && pa[0] != pc[0] {
            // let kab = (pb[1] - pa[1]) as f32 / (pb[0] - pa[0]) as f32;
            // let kbc = (pb[1] - pc[1]) as f32 / (pb[0] - pc[0]) as f32;
            // return kab != kbc;
            // compare slopes, transform division to multiplication
            return (pb[1] - pa[1]) * (pb[0] - pc[0]) != (pb[1] - pc[1]) * (pb[0] - pa[0]);
        }
        true
    }
}

fn main() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    println!("{:?} -> {}", points.clone(), Solution::is_boomerang(points));
}
