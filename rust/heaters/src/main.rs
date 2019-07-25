// Winter is coming! Your first job during the contest is
// to design a standard heater with fixed warm radius to warm all the houses.

// Now, you are given positions of houses and heaters on a horizontal line,
// find out minimum radius of heaters so that
// all houses could be covered by those heaters.

// So, your input will be the positions of houses and heaters seperately,
// and your expected output will be the minimum radius standard of heaters.

// Note:
// Numbers of houses and heaters you are given are
// non-negative and will not exceed 25000.
// Positions of houses and heaters you are given are
// non-negative and will not exceed 10^9.
// As long as a house is in the heaters' warm radius range, it can be warmed.
// All the heaters follow your radius standard and the warm radius will the same.

// Example 1:
// Input: [1,2,3],[2]
// Output: 1
// Explanation: The only heater was placed in the position 2,
// and if we use the radius 1 standard, then all the houses can be warmed.

// Example 2:
// Input: [1,2,3,4],[1,4]
// Output: 1
// Explanation: The two heater was placed in the position 1 and 4.
// We need to use radius 1 standard, then all the houses can be warmed.

struct Solution;

impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut radius: Vec<i32> = Vec::with_capacity(houses.len());
        let heater_num = heaters.len();
        let mut heaters = heaters.clone();
        heaters.sort();
        for h in houses {
            let mut l = 0;
            let mut r = heater_num - 1;
            while l < r {
                let m = l + (r - l) / 2;
                if h > heaters[m] {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            if h == heaters[l] {
                radius.push(0);
            } else if h > heaters[l] {
                radius.push(h - heaters[l]);
            } else if l > 0 {
                radius.push(std::cmp::min(heaters[l] - h, h - heaters[l - 1]));
            } else {
                radius.push(heaters[l] - h);
            }
        }
        radius.into_iter().max().unwrap()
    }
}

fn main() {
    let houses = vec![1, 2, 3, 4];
    let heaters = vec![1, 4];
    println!(
        "{:?}, {:?} -> {}",
        houses.clone(),
        heaters.clone(),
        Solution::find_radius(houses, heaters)
    );
}
