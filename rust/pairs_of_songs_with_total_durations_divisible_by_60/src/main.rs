// In a list of songs, the i-th song has a duration of time[i] seconds.
// Return the number of pairs of songs for which their total duration in seconds is divisible by 60.
// Formally, we want the number of indices i < j with (time[i] + time[j]) % 60 == 0.

// Example 1:
// Input: [30,20,150,100,40]
// Output: 3
// Explanation: Three pairs have a total duration divisible by 60:
// (time[0] = 30, time[2] = 150): total duration 180
// (time[1] = 20, time[3] = 100): total duration 120
// (time[1] = 20, time[4] = 40): total duration 60

// Example 2:
// Input: [60,60,60]
// Output: 3
// Explanation: All three pairs have a total duration of 120, which is divisible by 60.

// Note:
// 1 <= time.length <= 60000
// 1 <= time[i] <= 500

struct Solution;

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut ret = 0;
        let len = time.len();
        if 2 > len {
            return ret;
        }
        let mut count = vec![0; 60];
        let mut timec = vec![0; len];
        for i in 0..len {
            timec[i] = time[i] % 60;
            count[timec[i] as usize] += 1;
        }
        timec.sort();
        for e in timec {
            if 0 == e {
                continue;
            }
            if 30 <= e {
                break;
            }
            ret += count[60 - e as usize];
        }
        ret += count[0] * (count[0] - 1) / 2;
        ret += count[30] * (count[30] - 1) / 2;
        ret
    }
}

fn main() {
    let time = vec![30, 20, 150, 100, 40];
    let timec = time.clone();
    println!(
        "{:?} -> {}",
        timec,
        Solution::num_pairs_divisible_by60(time)
    );
}
