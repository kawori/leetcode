// You're given strings J representing the types of stones that are jewels, and S representing the stones you have.  Each character in S is a type of stone you have.  You want to know how many of the stones you have are also jewels.
// The letters in J are guaranteed distinct, and all characters in J and S are letters. Letters are case sensitive, so "a" is considered a different type of stone from "A".

// Example 1:
// Input: J = "aA", S = "aAAbbbb"
// Output: 3

// Example 2:
// Input: J = "z", S = "ZZ"
// Output: 0

// Note:
// S and J will consist of letters and have length at most 50.
// The characters in J are distinct.

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut cnt = vec![0; 52];
        for b in s.chars() {
            if b.is_ascii_lowercase() {
                cnt[b as usize - 'a' as usize] += 1;
            } else {
                cnt[b as usize - 'A' as usize + 26] += 1;
            }
        }
        let mut ret = 0;
        for b in j.chars() {
            if b.is_lowercase() {
                ret += cnt[b as usize - 'a' as usize];
            } else {
                ret += cnt[b as usize - 'A' as usize + 26];
            }
        }
        ret
    }
}

fn main() {
    let j = String::from("aA");
    let s = String::from("aAAbbbb");
    let jc = j.clone();
    let sc = s.clone();
    println!("{}, {} -> {}", jc, sc, Solution::num_jewels_in_stones(j, s));
}
