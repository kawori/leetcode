// A valid parentheses string is either empty (""), "(" + A + ")", or A + B, where A and B are valid parentheses strings, and + represents string concatenation.  For example, "", "()", "(())()", and "(()(()))" are all valid parentheses strings.
// A valid parentheses string S is primitive if it is nonempty, and there does not exist a way to split it into S = A+B, with A and B nonempty valid parentheses strings.
// Given a valid parentheses string S, consider its primitive decomposition: S = P_1 + P_2 + ... + P_k, where P_i are primitive valid parentheses strings.
// Return S after removing the outermost parentheses of every primitive string in the primitive decomposition of S.

// Example 1:
// Input: "(()())(())"
// Output: "()()()"
// Explanation:
// The input string is "(()())(())", with primitive decomposition "(()())" + "(())".
// After removing outer parentheses of each part, this is "()()" + "()" = "()()()".

// Example 2:
// Input: "(()())(())(()(()))"
// Output: "()()()()(())"
// Explanation:
// The input string is "(()())(())(()(()))", with primitive decomposition "(()())" + "(())" + "(()(()))".
// After removing outer parentheses of each part, this is "()()" + "()" + "()(())" = "()()()()(())".

// Example 3:
// Input: "()()"
// Output: ""
// Explanation:
// The input string is "()()", with primitive decomposition "()" + "()".
// After removing outer parentheses of each part, this is "" + "" = "".

// Note:
// S.length <= 10000
// S[i] is "(" or ")"
// S is a valid parentheses string

struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack: Vec<u8> = Vec::new();
        let mut sep: Vec<i32> = vec![-1];
        for (i, b) in s.as_bytes().iter().enumerate() {
            if *b == b'(' {
                stack.push(b'(');
            } else {
                stack.pop();
            }
            if stack.is_empty() {
                sep.push(i as i32);
            }
        }
        let num = sep.len() - 1;
        let mut i: usize = 0;
        let mut ret = String::new();
        while i < num {
            ret.push_str(&s[((sep[i] + 2) as usize)..(sep[i + 1] as usize)]);
            i += 1;
        }
        ret
    }
}

fn main() {
    let parentheses = String::from("(()())(())");
    let pc = parentheses.clone();
    println!(
        "{} -> {}",
        pc,
        Solution::remove_outer_parentheses(parentheses)
    );
}
