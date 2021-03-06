// In a deck of cards, each card has an integer written on it.

// Return true if and only if you can choose X >= 2 such that
// it is possible to split the entire deck into 1 or more groups of cards, where:

// Each group has exactly X cards.
// All the cards in each group have the same integer.

// Example 1:
// Input: [1,2,3,4,4,3,2,1]
// Output: true
// Explanation: Possible partition [1,1],[2,2],[3,3],[4,4]

// Example 2:
// Input: [1,1,1,2,2,2,3,3]
// Output: false
// Explanation: No possible partition.

// Example 3:
// Input: [1]
// Output: false
// Explanation: No possible partition.

// Example 4:
// Input: [1,1]
// Output: true
// Explanation: Possible partition [1,1]

// Example 5:
// Input: [1,1,2,2,2,2]
// Output: true
// Explanation: Possible partition [1,1],[2,2],[2,2]

// Note:
// 1 <= deck.length <= 10000
// 0 <= deck[i] < 10000

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn gcd(a: i32, b: i32) -> i32 {
        if 0 == b {
            return a;
        }
        let mut l;
        let mut r;
        if a > b {
            l = a % b;
            r = b;
        } else {
            l = b % a;
            r = a;
        }
        while l != 0 {
            let tmp = l;
            l = r % l;
            r = tmp;
        }
        r
    }
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for d in deck {
            if let Some(x) = map.get_mut(&d) {
                *x += 1;
            } else {
                map.insert(d, 1);
            }
        }
        let mut gcd = 0;
        for c in map.values() {
            gcd = Solution::gcd(*c, gcd);
        }
        gcd >= 2
    }
}

fn main() {
    let cards = vec![1, 2, 3, 4, 4, 3, 2, 1];
    println!(
        "{:?} -> {}",
        cards.clone(),
        Solution::has_groups_size_x(cards)
    );
}
