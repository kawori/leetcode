// Given an array of integers and an integer k,
// find out whether there are two distinct indices i and j in the array
// such that nums[i] = nums[j]
// and the absolute difference between i and j is at most k.

// Example 1:
// Input: nums = [1,2,3,1], k = 3
// Output: true

// Example 2:
// Input: nums = [1,0,1,1], k = 1
// Output: true

// Example 3:
// Input: nums = [1,2,3,1,2,3], k = 2
// Output: false

#include <iostream>
#include <map>
#include <vector>
using std::map;
using std::vector;

class Solution {
public:
  bool containsNearbyDuplicate(vector<int> &nums, int k) {
    map<int, int> my_map;
    int len = nums.size(), n;
    for (int i = 0; i < len; ++i) {
      n = nums[i];
      if (my_map.count(n)) {
        if (i - my_map[n] <= k) {
          return true;
        } else {
          my_map[n] = i;
        }
      } else {
        my_map[n] = i;
      }
    }
    return false;
  }
};

int main() {
  Solution sol;
  vector<int> nums1{1, 2, 3, 1};
  std::cout << "[ 1, 2, 3, 1 ], 3 -> ";
  if (sol.containsNearbyDuplicate(nums1, 3)) {
    std::cout << "true\n";
  } else {
    std::cout << "false\n";
  }

  vector<int> nums2{1, 2, 3, 1, 2, 3};
  std::cout << "[ 1, 2, 3, 1, 2, 3 ], 2 -> ";
  if (sol.containsNearbyDuplicate(nums2, 2)) {
    std::cout << "true\n";
  } else {
    std::cout << "false\n";
  }

  vector<int> nums3{1, 0, 1, 1};
  std::cout << "[ 1, 0, 1, 1 ], 1 -> ";
  if (sol.containsNearbyDuplicate(nums3, 1)) {
    std::cout << "true\n";
  } else {
    std::cout << "false\n";
  }

  return 0;
}
