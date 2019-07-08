// Given a non-empty array of digits representing a non-negative integer, plus one to the integer.

// The digits are stored such that the most significant digit is at the head of the list, and each element in the array contain a single digit.

// You may assume the integer does not contain any leading zero, except the number 0 itself.

// Example 1:
// Input: [1,2,3]
// Output: [1,2,4]
// Explanation: The array represents the integer 123.

// Example 2:
// Input: [4,3,2,1]
// Output: [4,3,2,2]
// Explanation: The array represents the integer 4321.

struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let width = digits.len();
        let mut i: usize = width - 1;
        let mut plus = true;
        let mut ret: Vec<i32> = Vec::new();
        let mut digits_cpy = digits;
        while plus {
            if 9 == digits_cpy[i] {
                digits_cpy[i] = 0;
                if i >= 1 {
                    i -= 1;
                } else {
                    ret.push(1);
                    break;
                }
            } else {
                digits_cpy[i] += 1;
                plus = false;
            }
        }
        for d in digits_cpy {
            ret.push(d);
        }
        ret
    }
}

fn main() {
    let l = vec![9, 9, 9];
    let lc = l.clone();
    let ln = Solution::plus_one(l);
    println!("{:?} -> {:?}", lc, ln);
}
