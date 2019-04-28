// https://leetcode-cn.com/problems/two-sum/
// Given an array of integers, return indices of the two numbers such that they
// add up to a specific target. You may assume that each input would have
// exactly one solution, and you may not use the same element twice. Example:
// Given nums = [2, 7, 11, 15], target = 9,

// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].

// Author: kawori

#include <map>
#include <vector>

class Solution {
public:
  std::vector<int> twoSum(std::vector<int> &nums, int target) {
    std::map<int, int> my_map;
    int len = nums.size(), s;
    std::vector<int> ret;
    for (int i = 0; i < len; ++i) {
      s = target - nums[i];
      if (my_map.count(s)) {
        ret.push_back(i);
        ret.push_back(my_map[s]);
        break;
      } else {
        my_map[nums[i]] = i;
      }
    }
    return ret;
  }
};