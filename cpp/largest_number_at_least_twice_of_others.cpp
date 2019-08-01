// In a given integer array nums, there is always exactly one largest element.
// Find whether the largest element in the array is
// at least twice as much as every other number in the array.
// If it is, return the index of the largest element, otherwise return -1.

// Example 1:
// Input: nums = [3, 6, 1, 0]
// Output: 1
// Explanation:
// 6 is the largest integer, and for every other number in the array x,
// 6 is more than twice as big as x. The index of value 6 is 1, so we return 1.

// Example 2:
// Input: nums = [1, 2, 3, 4]
// Output: -1
// Explanation:
// 4 isn't at least as big as twice the value of 3, so we return -1.

// Note:
// nums will have a length in the range [1, 50].
// Every nums[i] will be an integer in the range [0, 99].

#include <vector>
using std::vector;
#include "utils.h"

class Solution {
public:
  int dominantIndex(vector<int> &nums) {
    int len = nums.size();
    if (2 > len) {
      return 0;
    }
    int max = 0, less = 0, ret = 0;
    for (int i = 0; i < len; ++i) {
      if (nums[i] > max) {
        less = max;
        max = nums[i];
        ret = i;
      } else if (nums[i] > less) {
        less = nums[i];
      }
    }
    return (max >= 2 * less) ? ret : -1;
  }
};

int main() {
  Solution sol;
  vector<int> nums{0, 0, 2, 3};
  std::cout << nums << " -> " << sol.dominantIndex(nums) << "\n";

  nums = vector<int>{3, 6, 1, 0};
  std::cout << nums << " -> " << sol.dominantIndex(nums) << "\n";

  return 0;
}
