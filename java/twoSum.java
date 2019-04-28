// https://leetcode-cn.com/problems/two-sum/
// Given an array of integers, return indices of the two numbers such that they
// add up to a specific target. You may assume that each input would have
// exactly one solution, and you may not use the same element twice. Example:
// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

// Author: kawori

import java.util.HashMap;

class Solution {
    public int[] twoSum(int[] nums, int target) {
        int len = nums.length;
        HashMap<Integer, Integer> my_map = new HashMap<>();
        int[] ret = { 0, 0 };
        for (int i = 0; i < len; ++i) {
            int complement = target - nums[i];
            if (my_map.containsKey(complement)) {
                ret[0] = i;
                ret[1] = my_map.get(complement);
                break;
            } else {
                my_map.put(nums[i], i);
            }
        }
        return ret;
    }
}