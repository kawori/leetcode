// Given two arrays arr1 and arr2, the elements of arr2 are distinct,
// and all elements in arr2 are also in arr1.

// Sort the elements of arr1 such that
// the relative ordering of items in arr1 are the same as in arr2.
// Elements that don't appear in arr2 should
// be placed at the end of arr1 in ascending order.

// Example 1:
// Input: arr1 = [2,3,1,3,2,4,6,7,9,2,19], arr2 = [2,1,4,3,9,6]
// Output: [2,2,2,1,4,3,3,9,6,7,19]

// Constraints:
// arr1.length, arr2.length <= 1000
// 0 <= arr1[i], arr2[i] <= 1000
// Each arr2[i] is distinct.
// Each arr2[i] is in arr1.

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = arr2.iter().map(|x| (*x, 0)).collect();
        let len1 = arr1.len();
        let mut ret: Vec<i32> = vec![0; len1];
        let mut remains: Vec<i32> = Vec::new();
        for elem in arr1 {
            if let Some(x) = map.get_mut(&elem) {
                *x += 1;
            } else {
                remains.push(elem);
            }
        }
        let mut beg = 0;
        for elem in arr2 {
            let cnt = map[&elem];
            let mut i = 0;
            while i < cnt {
                ret[beg + i] = elem;
                i += 1;
            }
            beg += cnt;
        }
        remains.sort();
        for i in beg..len1 {
            ret[i] = remains[i - beg];
        }
        ret
    }
}

fn main() {
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    println!(
        "{:?}, {:?} -> {:?}",
        arr1.clone(),
        arr2.clone(),
        Solution::relative_sort_array(arr1, arr2)
    );
}
