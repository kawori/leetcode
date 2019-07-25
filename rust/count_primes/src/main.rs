// Count the number of prime numbers less than a non-negative number, n.

// Example:
// Input: 10
// Output: 4
// Explanation: There are 4 prime numbers less than 10, they are 2, 3, 5, 7.

// Sieve of Eratosthenes

struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let n = n as usize;
        let mut primes = vec![true;n];
        primes[0] = false;
        primes[1] = false;
        let mut i: usize = 4;
        while i < n {
            primes[i] = false;
            i += 2;
        }
        i = 3;
        while i * i < n {
            if primes[i] {
                let mut j = i * i;
                while j < n {
                    primes[j] = false;
                    j += 2 * i; // i is always odd, no need to j += i
                }
            }
            i += 1;
        }
        primes.into_iter().filter(|&b| b == true).count() as i32
    }
}

fn main() {
    println!("10 -> {}", Solution::count_primes(10));
}
