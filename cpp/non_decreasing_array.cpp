// Given an array with n integers, your task is to check
// if it could become non-decreasing by modifying at most 1 element.

// We define an array is non-decreasing
// if array[i] <= array[i + 1] holds for every i (1 <= i < n).

// Example 1:
// Input: [4,2,3]
// Output: True
// Explanation: You could modify the first 4 to 1 to get a non-decreasing array.

// Example 2:
// Input: [4,2,1]
// Output: False
// Explanation:
// You can't get a non-decreasing array by modify at most one element.

// Note: The n belongs to [1, 10,000].

#include <iostream>
#include <vector>
using std::vector;

class Solution {
public:
  bool checkPossibility(vector<int> &nums) {
    int len = nums.size();
    if (3 > len) {
      return true;
    }
    int cnt = 0;
    if (nums[0] > nums[1]) {
      nums[0] = nums[1];
      cnt++;
    }
    for (int i = 1; i < len - 1; ++i) {
      if (nums[i] > nums[i + 1]) {
        if (1 == cnt) {
          return false;
        }
        if (nums[i - 1] <= nums[i + 1]) {
          nums[i] = nums[i - 1];
        } else {
          nums[i + 1] = nums[i];
        }
        cnt++;
      }
    }
    return true;
  }
};

int main() {
  Solution sol;
  vector<int> vec{4, 2, 3};
  std::cout << "[ 4, 2, 3 ] -> " << sol.checkPossibility(vec) << "\n";
  vector<int> nums{4, 2, 1};
  std::cout << "[ 4, 2, 1 ] -> " << sol.checkPossibility(nums) << "\n";
  return 0;
}
