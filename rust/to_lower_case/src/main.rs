// Implement function ToLowerCase() that has a string parameter str, and returns the same string in lowercase.

// Example 1:
// Input: "Hello"
// Output: "hello"

// Example 2:
// Input: "here"
// Output: "here"

// Example 3:
// Input: "LOVELY"
// Output: "lovely"

struct Solution;

impl Solution {
    // pub fn to_lower_case(str: String) -> String {
    //     let mut ret = str;
    //     unsafe {
    //         for ch in ret.as_mut_vec() {
    //             if *ch >= b'A' && *ch <= b'Z' {
    //                 *ch += b'a' - b'A';
    //             }
    //         }
    //     }
    //     ret
    // }

    pub fn to_lower_case(str: String) -> String {
        let mut ret = String::from("");
        let gap = b'a' - b'A';
        for ch in str.as_bytes() {
            if *ch >= b'A' && *ch <= b'Z' {
                ret.push((*ch + gap) as char);
            } else {
                ret.push(*ch as char);
            }
        }
        ret
    }
}

fn main() {
    let s = String::from("Hello");
    let sc = s.clone();
    println!("{} -> {}", sc, Solution::to_lower_case(s));
}
