// Write a function to
// find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:
// Input: ["flower","flow","flight"]
// Output: "fl"

// Example 2:
// Input: ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Note:
// All given inputs are in lowercase letters a-z.

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }
        let mut len: usize = strs[0].len();
        let mut i: usize = 1;
        let str_num = strs.len();
        while i != str_num {
            if strs[i].len() < len {
                len = strs[i].len();
            }
            i += 1;
        }
        i = 0;
        while i != len {
            let c = strs[0].as_bytes()[i];
            let mut j: usize = 1;
            while j != str_num {
                if c != strs[j].as_bytes()[i] {
                    return strs[0][..i].to_string();
                }
                j += 1;
            }
            i += 1;
        }
        strs[0][..len].to_string()
    }
}

fn main() {
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    println!(
        "[flower, flow, flight]: {}",
        Solution::longest_common_prefix(strs)
    );
    let strs = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    println!(
        "[dog, racecar, car]: {}",
        Solution::longest_common_prefix(strs)
    );
}
