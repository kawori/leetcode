// Given two integers L and R,
// find the count of numbers in the range [L, R] (inclusive)
// having a prime number of set bits in their binary representation.

// (Recall that the number of set bits an integer has
// is the number of 1s present when written in binary.
// For example, 21 written in binary is 10101 which has 3 set bits.
// Also, 1 is not a prime.)

// Example 1:

// Input: L = 6, R = 10
// Output: 4
// Explanation:
// 6 -> 110 (2 set bits, 2 is prime)
// 7 -> 111 (3 set bits, 3 is prime)
// 9 -> 1001 (2 set bits , 2 is prime)
// 10->1010 (2 set bits , 2 is prime)

// Example 2:

// Input: L = 10, R = 15
// Output: 5
// Explanation:
// 10 -> 1010 (2 set bits, 2 is prime)
// 11 -> 1011 (3 set bits, 3 is prime)
// 12 -> 1100 (2 set bits, 2 is prime)
// 13 -> 1101 (3 set bits, 3 is prime)
// 14 -> 1110 (3 set bits, 3 is prime)
// 15 -> 1111 (4 set bits, 4 is not prime)

// Note:

// L, R will be integers L <= R in the range [1, 10^6].
// R - L will be at most 10000.

struct Solution;

impl Solution {
    pub fn set_bits(n: i32) -> i32 {
        let mut ret = 0;
        let mut x = n;
        while x != 0 {
            if (x & 1) != 0 {
                ret += 1;
            }
            x >>= 1;
        }
        ret
    }
    pub fn count_prime_set_bits(l: i32, r: i32) -> i32 {
        let mut ret = 0;
        let mut bit_cnt = 0;
        for i in l..r + 1 {
            // let bit_cnt = format!("{:b}", i).chars().filter(|c| *c == '1').count();
            if i == l || (i & 1) == 0 {
                bit_cnt = Solution::set_bits(i);
            } else {
                bit_cnt += 1; // bits[odd] = bits[even] + 1
            }
            if bit_cnt == 2
                || bit_cnt == 3
                || bit_cnt == 5
                || bit_cnt == 7
                || bit_cnt == 11
                || bit_cnt == 13
                || bit_cnt == 17
                || bit_cnt == 19
            {
                ret += 1;
            }
        }
        ret
    }
}

fn main() {
    println!("L = 6, R = 10 -> {}", Solution::count_prime_set_bits(6, 10));
    println!(
        "L = 10, R = 15 -> {}",
        Solution::count_prime_set_bits(10, 15)
    );
}
