use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_map: HashMap<i32, usize> = HashMap::new();
        let length = nums.len();
        let mut i = 0;
        let mut ret: Vec<i32> = Vec::new();
        while i < length {
            let complement = target - nums[i];
            if my_map.contains_key(&complement) {
                ret.push(i as i32);
                let s = my_map.get(&complement);
                match s {
                    Some(&value) => {ret.push(value as i32);}
                    None => (),
                }
            } else {
                my_map.insert(&nums[i], &i);
            }
            i += 1;
        }
        return ret;
    }
}