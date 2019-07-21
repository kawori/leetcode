// A self-dividing number is a number that is divisible by every digit it contains.
// For example, 128 is a self-dividing number because 128 % 1 == 0, 128 % 2 == 0, and 128 % 8 == 0.
// Also, a self-dividing number is not allowed to contain the digit zero.
// Given a lower and upper number bound, output a list of every possible self dividing number, including the bounds if possible.

// Example 1:
// Input:
// left = 1, right = 22
// Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]

// Note:
// The boundaries of each input argument are 1 <= left <= right <= 10000.

struct Solution;

impl Solution {
    pub fn is_self_dividing_numbers2(n: i32) -> bool {
        let mut digits: Vec<i32> = Vec::new();
        let mut tmp = n;
        while tmp != 0 {
            let remainder = tmp % 10;
            if 0 == remainder {
                return false;
            } else {
                digits.push(remainder);
            }
            tmp /= 10;
        }
        for i in digits {
            if n % i != 0 {
                return false;
            }
        }
        true
    }
    pub fn is_self_dividing_numbers(n: i32) -> bool {
        let digits = n.to_string();
        for d in digits.as_bytes() {
            if *d == b'0' {
                return false;
            } else {
                let i = (*d - b'0') as i32;
                if n % i != 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        for i in left..right + 1 {
            if Solution::is_self_dividing_numbers2(i) {
                ret.push(i);
            }
        }
        ret
    }
}

fn main() {
    println!(
        "left: {}, right: {} -> {:?}",
        1,
        22,
        Solution::self_dividing_numbers(1, 22)
    );
}
